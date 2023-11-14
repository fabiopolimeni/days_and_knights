use ambient_api::{
    animation::{self, AnimationPlayerRef, BindId, BlendNodeRef, PlayClipFromUrlNodeRef, PlayMode},
    core::{
        animation::components::apply_animation_player,
        model::components::model_from_url,
        player::components::is_player,
        transform::concepts::{Transformable, TransformableOptional},
    },
    prelude::*,
    rand,
};

use packages::this::assets;

mod hero;

#[main]
pub fn main() {

    spawn_query(is_player()).bind(move |players| {
        let heros = [
            hero::Class::Barbarian,
            hero::Class::Knight,
            hero::Class::Mage,
            hero::Class::Rogue,
        ];

        // For each player that joins, spawn a random hero
        for (id, _) in players {
            let mut rng = rand::thread_rng();
            let hero = heros[rng.gen_range(0..heros.len())];
            let hero_str = hero.to_string();


            let idle_clip = PlayClipFromUrlNodeRef::new(
                assets::url(format!("characters/{}/{}.glb/animations/Idle_36.anim", hero_str, hero_str).as_str()),
            );

            entity::add_components(
                id,
                Entity::new()
                    .with(
                        model_from_url(),
                        assets::url(format!("characters/{}/{}.glb", hero_str, hero_str).as_str()),
                    )
                    .with_merge(Transformable {
                        local_to_world: Default::default(),
                        optional: TransformableOptional {
                            scale: Some(Vec3::ONE * 0.3),
                            ..Default::default()
                        },
                    }),
            );
        }
    });
}
