use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    core::{
        animation::components::apply_animation_player,
        ecs::components::remove_at_game_time,
        model::components::model_from_url,
        physics::components::plane_collider,
        player::components::is_player,
        primitives::components::{cube, quad},
        rendering::components::color,
        transform::{
            components::{rotation, scale, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    prelude::*,
    rand,
};

use hero::MAX_REMAINING_LOCOMOTION_TIME;
use packages::character_movement::concepts::*;
use packages::unit_schema::components::*;

use packages::this::assets;
use packages::this::components::*;
use packages::this::concepts::*;
use packages::this::messages::*;
use packages::this::types::*;

mod hero;

#[main]
pub async fn main() {
    let hero_classes = [
        hero::Class::Barbarian,
        hero::Class::Knight,
        hero::Class::Mage,
        hero::Class::Rogue,
    ];

    // Temporary ground
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 100.0)
        .with(color(), vec4(0.2, 0.2, 0.2, 1.0))
        .with(plane_collider(), ())
        .with(physics_layer(), PhysicsLayer::Ground)
        .spawn();

    // Load all hero animations.
    // Note: Because all heroes share the same animations, we can simply load those ones from the first hero.
    let idle_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Idle_36.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let walking_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Walking_B_73.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let running_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Running_A_48.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let jump_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Jump_Full_Short_39.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let drink_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Use_Item_71.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let attack_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/1H_Melee_Attack_Slice_Diagonal_1.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    let interact_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!(
            "characters/{}/{}.glb/animations/Interact_37.anim",
            hero_classes[0].to_string(),
            hero_classes[0].to_string()
        )
        .as_str(),
    ));

    idle_clip.wait_for_load().await;
    walking_clip.wait_for_load().await;
    running_clip.wait_for_load().await;
    jump_clip.wait_for_load().await;
    drink_clip.wait_for_load().await;
    attack_clip.wait_for_load().await;
    interact_clip.wait_for_load().await;

    jump_clip.looping(false);
    drink_clip.looping(false);
    attack_clip.looping(false);
    interact_clip.looping(false);

    let anim_player_idle = AnimationPlayerRef::new(&idle_clip);
    let anim_player_walk = AnimationPlayerRef::new(&walking_clip);
    let anim_player_run = AnimationPlayerRef::new(&running_clip);
    let anim_player_jump = AnimationPlayerRef::new(&jump_clip);
    let anim_player_drink = AnimationPlayerRef::new(&drink_clip);
    let anim_player_attack = AnimationPlayerRef::new(&attack_clip);
    let anim_player_interact = AnimationPlayerRef::new(&interact_clip);

    // Spawn a skeleton
    let skeleton = Entity::new()
        .with(
            model_from_url(),
            assets::url("skeletons/Mage/character_skeleton_mage.glb"),
        )
        .with_merge(Transformable {
            local_to_world: Default::default(),
            optional: TransformableOptional {
                translation: Some(vec3(4.0, 2.0, 0.0)),
                ..default()
            },
        })
        .with(apply_animation_player(), anim_player_idle.0)
        .with(physics_layer(), PhysicsLayer::Character)
        .spawn();

    spawn_query(is_player()).bind(move |players| {
        // For each player that joins, spawn a random hero
        for (player_id, _) in players {
            let mut rng = rand::thread_rng();
            let hero = hero_classes[rng.gen_range(0..hero_classes.len())];
            let hero_str = hero.to_string();

            entity::add_components(
                player_id,
                Entity::new()
                    .with(
                        model_from_url(),
                        assets::url(format!("characters/{}/{}.glb", hero_str, hero_str).as_str()),
                    )
                    .with(apply_animation_player(), anim_player_idle.0)
                    .with(physics_layer(), PhysicsLayer::Character)
                    .with(locomotion_remaining_time(), 0.0)
                    .with(game_timestamp(), game_time())
                    .with_merge(Transformable {
                        local_to_world: Default::default(),
                        optional: TransformableOptional::default(),
                    })
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
                        optional: CharacterMovementOptional {
                            run_speed_multiplier: Some(hero::SPEED_MULTIPLIER),
                            speed: Some(0.0),
                            ..default()
                        },
                    })
                    .with_merge(Hero::suggested()),
            );

            println!("Player {:?} joined as {}", player_id, hero);
        }
    });

    Movement::subscribe(|ctx, msg| {
        let player_id = ctx.client_entity_id().unwrap();
        //println!("Player {:?} sent {:?}", player_id, msg);

        if let Some(hit) = physics::raycast_first(msg.screen_ray_origin, msg.screen_ray_direction) {
            if entity::get_component(hit.entity, physics_layer()).unwrap_or_default()
                != PhysicsLayer::Ground
            {
                return;
            }

            // DEBUG: Remove this debug visualization
            Entity::new()
                .with(cube(), ())
                .with(translation(), hit.position)
                .with(scale(), Vec3::ONE * 0.1)
                .with(color(), vec4(0., 1., 0., 1.))
                .with(remove_at_game_time(), game_time() + Duration::from_secs(2))
                .spawn();

            let cur_pos = entity::get_component(player_id, translation()).unwrap_or_default();
            let move_diff = hit.position - cur_pos;

            // Find the direction the player is looking at in World space
            let look_dir_xy = (hit.position - cur_pos).xy().normalize();

            // Find the forward direction of the player in World space
            let cur_orientation = entity::get_component(player_id, rotation()).unwrap_or_default();
            let fwd_dir_xy = (cur_orientation * Vec3::Y).xy().normalize();

            // Find the rotation that rotates the player to look at the target
            let move_rot = Quat::from_rotation_arc_2d(fwd_dir_xy, look_dir_xy);

            // Apply the rotation to current player orientation
            let cur_rot = move_rot * cur_orientation;
            entity::set_component(player_id, rotation(), cur_rot);

            // Only move if the player is not too close to the target
            if move_diff.length_squared() >= hero::MIN_MOVE_DISTANCE {
                // In order to make the movement velocity independent of the frame rate,
                // we need to multiply the speed by the delta time we last entered this function.
                // But, if the character was not moving, then this is the first time we enter this function,
                // so we need to use a default value for the delta time, which is going to be the fixed tick rate.
                let prv_speed = entity::get_component(player_id, speed()).unwrap_or_default();
                let prv_game_timestamp =
                    entity::get_component(player_id, game_timestamp()).unwrap_or_default();
                let speed_time = if prv_speed <= f32::EPSILON || prv_game_timestamp.is_zero() {
                    delta_time()
                }
                else {
                    (game_time() - entity::get_component(player_id, game_timestamp()).unwrap_or_default()).as_secs_f32()
                };

                // Find the direction the player is running in World space
                let run_dir_xy = (move_rot * Vec3::Y).xy().normalize();
                entity::set_component(player_id, run_direction(), run_dir_xy);
                entity::set_component(player_id, speed(), hero::SPEED * speed_time);
                entity::set_component(
                    player_id,
                    locomotion_remaining_time(),
                    MAX_REMAINING_LOCOMOTION_TIME,
                );
            }

            entity::set_component(player_id, game_timestamp(), game_time());
        }
    });

    change_query(locomotion_remaining_time())
        .track_change(locomotion_remaining_time())
        .bind(move |players| {
            for (player_id, cur_speed) in players {
                let locomotion_time = entity::get_component(player_id, locomotion_remaining_time())
                    .unwrap_or_default();

                let remaining_time = locomotion_time - delta_time();
                if remaining_time > 0.0 {
                    entity::set_component(player_id, locomotion_remaining_time(), remaining_time);
                    if cur_speed > 0.0 {
                        entity::set_component(player_id, moving(), true);
                        let is_running =
                            entity::get_component(player_id, running()).unwrap_or_default();
                        if is_running {
                            println!("Player {:?} is running", player_id);
                            entity::set_component(
                                player_id,
                                apply_animation_player(),
                                anim_player_run.0,
                            );
                        } else {
                            println!("Player {:?} is walking", player_id);
                            entity::set_component(
                                player_id,
                                apply_animation_player(),
                                anim_player_walk.0,
                            );
                        }
                    }
                } else if locomotion_time > f32::EPSILON {
                    println!("Player {:?} is idle", player_id);
                    entity::set_component(player_id, locomotion_remaining_time(), 0.0);
                    entity::set_component(player_id, speed(), 0.0);
                    entity::set_component(player_id, moving(), false);
                    entity::set_component(player_id, apply_animation_player(), anim_player_idle.0);
                }
            }
        });

    Action::subscribe(move |ctx, msg| {
        let player_id = ctx.client_entity_id().unwrap();
        println!("Player {:?} sent {:?}", player_id, msg);

        let is_on_ground = entity::get_component(player_id, is_on_ground()).unwrap_or_default();
        if msg.jump == true && is_on_ground {
            //entity::set_component(player_id, vertical_velocity(), 0.1);
            entity::set_component(player_id, jumping(), true);
            entity::set_component(player_id, apply_animation_player(), anim_player_jump.0);
            println!("Player {:?} is jumping", player_id);
        }

        if msg.sprint == true {
            entity::set_component(player_id, running(), true);
        } else if msg.sprint == false {
            entity::set_component(player_id, running(), false);
        }

        let is_moving = entity::get_component(player_id, moving()).unwrap_or_default();

        if msg.drink && !is_moving {
            drink_clip.restart();
            entity::set_component(player_id, apply_animation_player(), anim_player_drink.0);
            println!("Player {:?} is drinking", player_id);
        } else if msg.attack && !is_moving {
            attack_clip.restart();
            entity::set_component(player_id, apply_animation_player(), anim_player_attack.0);
            println!("Player {:?} is attacking", player_id);
        } else if msg.interact && !is_moving {
            interact_clip.restart();
            entity::set_component(player_id, apply_animation_player(), anim_player_interact.0);
            println!("Player {:?} is interacting", player_id);
        }
    });
}
