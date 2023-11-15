use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    core::{
        animation::components::apply_animation_player,
        app::components::name,
        hierarchy::components::children,
        model::components::model_from_url,
        physics::{components::{plane_collider, visualize_collider}, concepts::CharacterController},
        player::components::is_player,
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::{
            components::{scale, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    entity::get_component,
    prelude::*,
    rand,
};

use packages::character_movement::concepts::*;
use packages::unit_schema::components::*;

use packages::this::assets;
use packages::this::messages::*;

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
                    .with(visualize_collider(), ())
                    .with(apply_animation_player(), anim_player.0)
                    .with_merge(CharacterMovement {
                        character_controller_height: 3.0,
                        character_controller_radius: 1.0,
                        physics_controlled: (),
                        rotation: Quat::IDENTITY,
                        run_direction: Vec2::ZERO,
                        vertical_velocity: 0.0,
                        running: false,
                        jumping: false,
                        is_on_ground: true,
                        optional: CharacterMovementOptional::default(),
                    }),
            );

            println!(
                "Player {:?} has children {:?}",
                player_id,
                entity::get_component(player_id, children())
            );
            for child_id in entity::get_component(player_id, children()).unwrap_or_default() {
                let name = entity::get_component(child_id, name());
                println!("Child {:?} has name {:?}", child_id, name)
            }

            println!("Player {:?} joined as {}", player_id, hero);
        }
    });

    Movement::subscribe(|ctx, msg| {
        let player_id = ctx.client_entity_id().unwrap();
        println!("Player {:?} sent {:?}", player_id, msg);

        // let Some(hit) = physics::raycast_first(msg.screen_ray_origin, msg.screen_ray_direction)
        // else {
        //     return;
        // };

        // TODO: Check whether we have hit the ground

        // Entity::new()
        //     .with(cube(), ())
        //     .with(translation(), hit.position)
        //     .with(scale(), Vec3::ONE * 0.1)
        //     .with(color(), vec4(0., 1., 0., 1.))
        //     .spawn();
    });

    Action::subscribe(|ctx, msg| {
        let player_id = ctx.client_entity_id().unwrap();
        println!("Player {:?} sent {:?}", player_id, msg);

        println!("{:#?}", entity::get_component(player_id, vertical_velocity()));

        if entity::get_component(player_id, is_on_ground()).unwrap_or_default() {
            entity::set_component(player_id, vertical_velocity(), 0.2);
            entity::set_component(player_id, jumping(), true);
        }
    });
}
