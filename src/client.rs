use std::f32::consts::PI;

use ambient_api::{
    core::{
        app::components::main_scene,
        messages::Frame,
        rendering::components::{fog_density, light_ambient, light_diffuse, sky, sun},
        transform::components::rotation,
    },
    prelude::*,
};

use packages::this::messages::*;
use packages::{orbit_camera::concepts::OrbitCamera, this::messages::Movement};

#[main]
pub fn main() {
    OrbitCamera::suggested().spawn();

    Entity::new().with(sky(), ()).spawn();

    let sun = Entity::new()
        .with(sun(), 0.0)
        .with(rotation(), Quat::IDENTITY)
        .with(main_scene(), ())
        .with(light_diffuse(), Vec3::ONE * 1.0)
        .with(light_ambient(), Vec3::ONE * 0.1)
        .with(fog_density(), 0.)
        .spawn();

    Frame::subscribe(move |_| {
        let time = game_time().as_secs_f32();

        // Negate it to start from daylight
        let sun_speed = -0.2f32;

        let rot = Quat::from_axis_angle(vec3(0.0, 1.0, 0.5).normalize(), time * sun_speed);
        entity::set_component(sun, rotation(), rot);
        let (_, _, z) = rot.to_euler(glam::EulerRot::XYZ);

        if z < 0.0 && z > -PI {
            // It is day
            entity::set_component(sun, light_diffuse(), Vec3::ONE);
        } else {
            // It is night
            entity::set_component(sun, light_diffuse(), Vec3::ZERO);
        }
    });

    fixed_rate_tick(Duration::from_millis(20), move |_| {
        let Some(camera_id) = camera::get_active() else {
            return;
        };

        let mut move_input = Movement {
            screen_ray_origin: Vec3::ZERO,
            screen_ray_direction: Vec3::ZERO,
        };

        let input = input::get();

        if input.mouse_buttons.contains(&MouseButton::Left) {
            let screen_ray = camera::screen_position_to_world_ray(camera_id, input.mouse_position);
            move_input.screen_ray_origin = screen_ray.origin;
            move_input.screen_ray_direction = screen_ray.dir;
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

        let mut send_action = if !delta.keys.is_empty() {
            true
        } else {
            false
        };

        if delta.keys.contains(&KeyCode::LShift) {
            action_input.sprint = true;
        } else if delta.keys_released.contains(&KeyCode::LShift) {
            action_input.sprint = false;
            send_action = true;
        }

        if delta.keys.contains(&KeyCode::Space) {
            action_input.jump = true;
        }
        
        if delta.keys.contains(&KeyCode::S) {
            action_input.drink = true;
        }

        if delta.keys.contains(&KeyCode::A) {
            action_input.interact = true;
        }

        if delta.keys.contains(&KeyCode::D) {
            action_input.attack = true;
        }

        if send_action {
            action_input.send_server_unreliable();
        }
    });
}
