use std::f32::consts::PI;
use std::sync::atomic::{AtomicBool, Ordering};

use ambient_api::{
    core::{
        messages::Frame,
        rendering::components::{fog_density, light_ambient, light_diffuse, sky, sun},
        transform::components::{lookat_target, rotation, translation},
    },
    entity::get_component,
    prelude::*,
};

use packages::this::messages::*;

use packages::{
    orbit_camera::components::*, orbit_camera::concepts::*,
};

static JOINED: AtomicBool = AtomicBool::new(false);

const MAX_CAMERA_DISTANCE: f32 = 25.0;

#[main]
pub async fn main() {
    let camera = OrbitCamera {
        is_orbit_camera: (),
        optional: OrbitCameraOptional {
            camera_distance: Some(MAX_CAMERA_DISTANCE),
            camera_angle: Some(Vec2::new(PI, PI / 4.0)),
            ..default()
        },
    }
    .spawn();

    Entity::new().with(sky(), ()).spawn();

    // When a sun is spawned, add lighting components
    spawn_query(sun()).bind(move |suns| {
        let sun = suns[0].0;
        entity::add_component(sun, light_ambient(), Vec3::ONE * 0.1);
        entity::add_component(sun, light_diffuse(), Vec3::ONE);
        entity::add_component(sun, fog_density(), 0.0);
    });

    let join_request = ClientRequest {
        join: true,
        disconnect: false,
    };

    join_request.send_server_reliable();

    ServerResponse::subscribe(move |_ctx, msg| {
        if msg.accepted {
            JOINED.store(true, Ordering::Release);
            println!("Player {} joined server", player::get_local());
        } else {
            println!("Player {} failed to join server", player::get_local());
        }
    });

    Frame::subscribe(move |_| {
        if !JOINED.load(Ordering::Acquire) {
            return;
        }
    });

    // Update sun lighting components based on whether it is day or night
    query(sun()).each_frame(move |suns| {
        let rot = entity::get_component(suns[0].0, rotation()).unwrap_or_default();
        let (_, _, z) = rot.to_euler(glam::EulerRot::XYZ);

        if z < 0.0 && z > -PI {
            // It is day
            entity::set_component(suns[0].0, light_diffuse(), Vec3::ONE);
        } else {
            // It is night
            entity::set_component(suns[0].0, light_diffuse(), Vec3::ZERO);
        }
    });

    Frame::subscribe(move |_| {
        // Fix camera angle
        let mut angle = get_component(camera, camera_angle()).unwrap_or_default();
        angle.y = PI / 4.0;
        entity::set_component(camera, camera_angle(), angle);

        // Fix camera distance
        let mut distance = get_component(camera, camera_distance()).unwrap_or_default();
        distance = distance.clamp(0.0, MAX_CAMERA_DISTANCE);
        entity::set_component(camera, camera_distance(), distance);

        // Make the camera look at the hero
        let player_id = player::get_local();
        if entity::has_component(player_id, translation()) {
            let hero_pos = entity::get_component(player_id, translation()).unwrap_or_default();

            if entity::has_component(camera, lookat_target()) {
                entity::set_component(camera, lookat_target(), hero_pos);
            }
        }
    });

    // Sent movement input to the server
    fixed_rate_tick(Duration::from_millis(50), move |_| {
        let Some(camera_id) = camera::get_active() else {
            return;
        };

        let mut move_input = Movement {
            screen_ray_origin: Vec3::ZERO,
            screen_ray_direction: Vec3::ZERO,
            move_direction: Vec2::ZERO,
        };

        let input = input::get();

        if input.mouse_buttons.contains(&MouseButton::Left) {
            let screen_ray = camera::screen_position_to_world_ray(camera_id, input.mouse_position);
            move_input.screen_ray_origin = screen_ray.origin;
            move_input.screen_ray_direction = screen_ray.dir;
        }

        // Figure out what's the camera's forward vector
        let camera_position = entity::get_component(camera_id, translation()).unwrap_or_default();
        let target_position = entity::get_component(camera_id, lookat_target()).unwrap_or_default();
        let camera_forward = (target_position - camera_position).xy().normalize();
        
        let mut input_vector = Vec2::ZERO;
        if input.keys.contains(&KeyCode::W) {
            input_vector += Vec2::Y;
        }
        if input.keys.contains(&KeyCode::S) {
            input_vector -= Vec2::Y;
        }
        if input.keys.contains(&KeyCode::A) {
            input_vector -= Vec2::X;
        }
        if input.keys.contains(&KeyCode::D) {
            input_vector += Vec2::X;
        }
        
        if input_vector != Vec2::ZERO {
            println!("camera_forward: {:?}", camera_forward);
            let input_vector = input_vector.yx();
            let movement_rotated = Vec2::new(
                input_vector.x * camera_forward.x - input_vector.y * camera_forward.y,
                input_vector.x * camera_forward.y + input_vector.y * camera_forward.x,
            );
            move_input.move_direction = movement_rotated;
            move_input.send_server_unreliable();
        }
    });

    // Send input actions to the server
    Frame::subscribe(move |_| {
        // if !is_game_focused() {
        //     return;
        // }

        let (delta, _) = input::get_delta();

        let mut action_input = Action {
            attack: false,
            drink: false,
            interact: false,
            jump: false,
            sprint: false,
        };

        let mut send_action = false;

        if delta.keys.contains(&KeyCode::LShift) {
            action_input.sprint = true;
            send_action = true;
        } else if delta.keys_released.contains(&KeyCode::LShift) {
            action_input.sprint = false;
            send_action = true;
        }

        if delta.keys.contains(&KeyCode::Space) {
            action_input.jump = true;
            send_action = true;
        }

        if delta.keys.contains(&KeyCode::Q) {
            action_input.drink = true;
            send_action = true;
        }

        if delta.keys.contains(&KeyCode::E) {
            action_input.interact = true;
            send_action = true;
        }

        if delta.keys.contains(&KeyCode::X) {
            action_input.attack = true;
            send_action = true;
        }

        if send_action {
            action_input.send_server_reliable();
        }
    });
}
