pub mod player;

use crate::components::*;
use specs::world::Builder;

pub type PlayerController = player::PlayerController;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder.with(
        player::PlayerControllerSystem,
        "Player Controller System",
        &[],
    )
}

pub fn register_default(world: &mut specs::World) {
    world.register::<PlayerController>();
}

pub fn setup_scene(world: &mut specs::World) {
    let player = player::spawn_player(world);

    let tile1 = world
        .create_entity()
        .with(Drawable::new(String::from("ice_wide")))
        .with(Transform {
            position: nalgebra::Point3::new(1.5, 0.0, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
            ..Default::default()
        })
        .build();

    let _camera1 = world
        .create_entity()
        .with(Camera::new((1024.0, 768.0), 3.0))
        .with(Transform {
            position: nalgebra::Point3::new(0.0, 0.0, -2.0),
            rotation: nalgebra::UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            ..Default::default()
        })
        .build();

    // Setup collisions
    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        let player_collider = collision::ColliderBuilder::new()
            .bounds(nalgebra::Vector2::new(0.2, 0.3))
            .build(&mut collision_world, player);

        let tile_collider_1 = collision::ColliderBuilder::new()
            .bounds(nalgebra::Vector2::repeat(0.2))
            .build(&mut collision_world, tile1);

        world
            .write_storage::<Collider>()
            .insert(player, player_collider)
            .unwrap();

        world
            .write_storage::<Collider>()
            .insert(tile1, tile_collider_1)
            .unwrap();
    }
}
