use std::f32::consts::PI;

use ambient_api::{
    core::{
        messages::Frame,
        rendering::components::{fog_density, light_ambient, light_diffuse, sky, sun},
        transform::components::rotation,
    },
    prelude::*, entity::get_component,
};

use packages::{this::messages::*, orbit_camera::components::camera_angle};
use packages::{orbit_camera::concepts::OrbitCamera, orbit_camera::concepts::OrbitCameraOptional, this::messages::Movement};

#[main]
pub fn main() {
    let camera = OrbitCamera {
        is_orbit_camera: (),
        optional: OrbitCameraOptional {
            camera_distance: Some(25.0),
            camera_angle: Some(Vec2::new(PI, PI / 4.0)),
            ..default()
        },
    }
    .spawn();

    Entity::new().with(sky(), ()).spawn();

    spawn_query(sun()).bind(move |suns| {
        let sun = suns[0].0;
        entity::add_component(sun, light_ambient(), Vec3::ONE * 0.1);
        entity::add_component(sun, light_diffuse(), Vec3::ONE);
        entity::add_component(sun, fog_density(), 0.0);
    });

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
    });

    fixed_rate_tick(Duration::from_millis(100), move |_| {
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
