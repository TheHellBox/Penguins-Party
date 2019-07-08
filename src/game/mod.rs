pub mod camera_controller;
pub mod map_loader;
pub mod one_way_platforms;
pub mod pickable;
pub mod player;

use crate::components::*;
use specs::world::Builder;

pub type PlayerController = player::PlayerController;
pub type Pickable = pickable::Pickable;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(pickable::PickableSystem, "Pickable System", &[])
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
        .with(one_way_platforms::OneWaySystem, "One Way Platforms", &[])
}

pub fn register_default(world: &mut specs::World) {
    world.register::<PlayerController>();
    world.register::<Pickable>();
}

pub fn setup_scene(world: &mut specs::World) {
    let _player = player::spawn_player(
        world,
        na::Point2::new(0.0, 3.0),
        player::controls::InputDevice::Keyboard(),
    );

    let pickable = world
        .create_entity()
        .with(Drawable::new(String::from("dev")))
        .with(Pickable::new())
        .with(Physics::new())
        .with(Transform {
            position: na::Point3::new(1.0, 3.0, 0.0),
            size: na::Vector2::repeat(0.15),
            ..Default::default()
        })
        .with(Child {
            local_position: na::Point3::new(0.1, 0.0, 0.0),
            local_rotation: na::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            parent: None,
        })
        .build();

    let resolution = world
        .read_resource::<crate::render::window::WindowBuilderInfo>()
        .resolution;
    let _camera1 = world
        .create_entity()
        .with(Camera::new(resolution, 30.0))
        .with(Transform {
            position: na::Point3::new(0.0, 0.0, -2.0),
            rotation: na::UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            ..Default::default()
        })
        .build();

    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        let collider = collision::ColliderBuilder::new()
            .bounds(na::Vector2::new(0.15, 0.15))
            .build(&mut collision_world, pickable);

        world
            .write_storage::<Colliders>()
            .insert(pickable, collision::Colliders(vec![collider]))
            .unwrap();
    }
}

pub fn update(world: &mut specs::World) {
    player::player_spawner::update(world);
}
