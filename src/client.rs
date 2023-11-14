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
}
