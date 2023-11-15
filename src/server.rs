use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    core::{
        animation::components::apply_animation_player,
        model::components::model_from_url,
        player::components::is_player,
        transform::{concepts::{Transformable, TransformableOptional}, components::scale},
        physics::{concepts::CharacterController, components::plane_collider}, primitives::components::quad, rendering::components::color,
    },
    prelude::*,
    rand,
};

use packages::this::assets;

mod hero;

#[main]
pub async fn main() {
    let heros = [
        hero::Class::Barbarian,
        hero::Class::Knight,
        hero::Class::Mage,
        hero::Class::Rogue,
    ];

    // Temporary ground
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 10.0)
        .with(color(), vec4(1.0, 1.0, 1.0, 1.0))
        .with(plane_collider(), ())
        .spawn();

    // Load all hero animations.
    // Note: Because all heroes share the same animations, we can simply load those ones from the first hero.
    let idle_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Idle_36.anim",
            heros[0].to_string(),
            heros[0].to_string()
        )
        .as_str(),
    ));

    idle_clip.wait_for_load().await;

    let anim_player = AnimationPlayerRef::new(&idle_clip);

    spawn_query(is_player()).bind(move |players| {
        // For each player that joins, spawn a random hero
        for (player_id, _) in players {
            let mut rng = rand::thread_rng();
            let hero = heros[rng.gen_range(0..heros.len())];
            let hero_str = hero.to_string();

            entity::add_components(
                player_id,
                Entity::new()
                    .with(
                        model_from_url(),
                        assets::url(format!("characters/{}/{}.glb", hero_str, hero_str).as_str()),
                    )
                    .with_merge(Transformable {
                        local_to_world: Default::default(),
                        optional: TransformableOptional::default(),
                    })
                    .with(apply_animation_player(), anim_player.0)
                    .with_merge(CharacterController {
                        character_controller_height: 0.5,
                        character_controller_radius: 1.8,
                        physics_controlled: ()
                    }),
            );
        }
    });
}
