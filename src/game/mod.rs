pub mod camera_controller;
pub mod map_loader;
pub mod player;

use crate::components::*;
use specs::world::Builder;

pub type PlayerController = player::PlayerController;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(
            player::PlayerControllerSystem,
            "Player Controller System",
            &[],
        )
        .with(
            camera_controller::CameraController,
            "Camera Controller",
            &[],
        )
}

pub fn register_default(world: &mut specs::World) {
    world.register::<PlayerController>();
}

pub fn setup_scene(world: &mut specs::World) {
    let player = player::spawn_player(world);

    let _camera1 = world
        .create_entity()
        .with(Camera::new((1024.0, 768.0), 10.0))
        .with(Transform {
            position: na::Point3::new(0.0, 0.0, -2.0),
            rotation: na::UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            ..Default::default()
        })
        .build();

    // Setup collisions
    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        let player_collider = collision::ColliderBuilder::new()
            .bounds(na::Vector2::new(0.2, 0.3))
            .build(&mut collision_world, player);

        world
            .write_storage::<Collider>()
            .insert(player, player_collider)
            .unwrap();
    }
}
