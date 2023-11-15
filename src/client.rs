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

use packages::orbit_camera::concepts::OrbitCamera;
use packages::this::messages::Action;

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

    // Send input actions to the server
    fixed_rate_tick(Duration::from_millis(20), move |_| {
        let Some(camera_id) = camera::get_active() else {
            return;
        };

        let input = input::get();

        let mut action = Action {
            point_ray_origin: Vec3::ZERO,
            point_ray_direction: Vec3::ZERO,
            primary_attack: false,
            secondary_attack: false,
            health_potion: false,
            mana_potion: false,
            jump: false,
            interact: false,
            sprint: false,
        };

        if input.mouse_buttons.contains(&MouseButton::Left) {
            let screen_ray = camera::screen_position_to_world_ray(camera_id, input.mouse_position);
            action.point_ray_origin = screen_ray.origin;
            action.point_ray_direction = screen_ray.dir;

            if input.keys.contains(&KeyCode::LShift) {
                action.sprint = true;
            }
        }

        if input.keys.contains(&KeyCode::S) {
            action.health_potion = true;
        }
        else if input.keys.contains(&KeyCode::W) {
            action.mana_potion = true;
        }

        if input.keys.contains(&KeyCode::Space) {
            action.jump = true;
        }

        if input.keys.contains(&KeyCode::E) {
            action.interact = true;
        }

        if input.keys.contains(&KeyCode::D) {
            action.primary_attack = true;
        }

        if input.keys.contains(&KeyCode::A) {
            action.secondary_attack = true;
        }

        action.send_server_unreliable();
    });

}
