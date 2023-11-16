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

use packages::character_movement::concepts::*;
use packages::unit_schema::components::*;

use packages::this::assets;
use packages::this::components::physics_layer;
use packages::this::messages::*;
use packages::this::types::PhysicsLayer;

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
        .with(color(), vec4(0.1, 0.1, 0.1, 1.0))
        .with(plane_collider(), ())
        .with(physics_layer(), PhysicsLayer::Ground)
        .spawn();

    // Load all hero animations.
    // Note: Because all heroes share the same animations, we can simply load those ones from the first hero.
    let idle_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Idle_36.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let walking_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Walking_B_73.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let running_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Running_A_48.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let jump_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Jump_Full_Short_39.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let drink_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Use_Item_71.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let attack_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/1H_Melee_Attack_Slice_Diagonal_1.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

    let interact_clip = PlayClipFromUrlNodeRef::new(assets::url(
        format!("characters/{}/{}.glb/animations/Interact_37.anim", heros[0].to_string(), heros[0].to_string()).as_str()));

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
                    .with(apply_animation_player(), anim_player_idle.0)
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
                            air_speed_multiplier: Some(0.0),
                            strafe_speed_multiplier: Some(0.0),
                        },
                    })
                    .with(physics_layer(), PhysicsLayer::Character), //.with(visualize_collider(), ()),
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
                // Find the direction the player is running in World space
                let run_dir_xy = (move_rot * Vec3::Y).xy().normalize();
                entity::set_component(player_id, run_direction(), run_dir_xy);
                entity::set_component(player_id, speed(), hero::SPEED);
            }
        }
    });

    // Amazing trick to decrement character speed over time
    change_query(speed())
        .track_change(speed())
        .bind(move|players| {
            for (player_id, cur_speed) in players {
                let new_speed = (cur_speed - 0.01).clamp(0.0, hero::MAX_SPEED);

                // This query only traks whether the component has been set, it does not
                // track the actual value of the component whether it has changed or not.
                if new_speed == cur_speed {
                    continue;
                }

                entity::set_component(player_id, speed(), new_speed);

                // Apply the correct locomotion animation based on the current speed
                if new_speed > 0.0 {
                    let is_running =
                        entity::get_component(player_id, running()).unwrap_or_default();
                    if is_running {
                        entity::set_component(player_id, apply_animation_player(), anim_player_run.0);
                    } else {
                        entity::set_component(player_id, apply_animation_player(), anim_player_walk.0);
                    }
                } else {
                    println!("Player {:?} is idle", player_id);
                    entity::set_component(player_id, apply_animation_player(), anim_player_idle.0);
                }
            }
        });

    Action::subscribe(move|ctx, msg| {
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

        let speed = entity::get_component(player_id, speed()).unwrap_or_default();
        let is_moving = if speed < 0.001 { false } else { true };

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
