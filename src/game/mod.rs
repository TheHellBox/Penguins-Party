pub mod camera_controller;
pub mod map_loader;
pub mod player;
pub mod one_way_platforms;

use crate::components::*;
use specs::world::Builder;

pub type PlayerController = player::PlayerController;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(
            player::controls::ControlsSystem,
            "Player Controls System",
            &[],
        )
        .with(
            player::PlayerControllerSystem,
            "Player Controller System",
            &[],
        )
        .with(
            player::player_animation::PlayerAnimationSystem,
            "Player Animation System",
            &[],
        )
        .with(
            player::player_killer::PlayerKillerSystem,
            "Player Killer System",
            &[],
        )
        .with(
            camera_controller::CameraController,
            "Camera Controller",
            &[],
        )
        .with(
            one_way_platforms::OneWaySystem,
            "One Way Platforms",
            &[],
        )
}

pub fn register_default(world: &mut specs::World) {
    world.register::<PlayerController>();
}

pub fn setup_scene(world: &mut specs::World) {
    let _player = player::spawn_player(
        world,
        na::Point2::new(0.0, 3.0),
        player::controls::InputDevice::Keyboard(),
    );

    let _camera1 = world
        .create_entity()
        .with(Camera::new((1024.0, 768.0), 30.0))
        .with(Transform {
            position: na::Point3::new(0.0, 0.0, -2.0),
            rotation: na::UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            ..Default::default()
        })
        .build();
}

pub fn update(world: &mut specs::World) {
    player::player_spawner::update(world);
}
