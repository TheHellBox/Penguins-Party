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
    let player = world
        .create_entity()
        .with(Drawable::new(String::from("penguin")))
        .with(Transform {
            position: nalgebra::Point2::new(0.0, 2.0),
            size: nalgebra::Vector2::new(0.3, 0.3),
            ..Default::default()
        })
        .with(PlayerController {})
        .with(Physics::new())
        .with(AnimationController::new(16).frame_update_speed(50))
        .build();

    let tile1 = world
        .create_entity()
        .with(Drawable::new(String::from("ice_wide")))
        .with(Transform {
            position: nalgebra::Point2::new(1.5, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
            ..Default::default()
        })
        .build();

    let tile2 = world
        .create_entity()
        .with(Drawable::new(String::from("ice_wide")))
        .with(Transform {
            position: nalgebra::Point2::new(-1.5, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
            ..Default::default()
        })
        .build();

    let tile3 = world
        .create_entity()
        .with(Drawable::new(String::from("ice_platform_wide")))
        .with(Transform {
            position: nalgebra::Point2::new(0.0, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
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
        let tile_collider_2 = collision::ColliderBuilder::new()
            .bounds(nalgebra::Vector2::repeat(0.2))
            .build(&mut collision_world, tile2);
        let tile_collider_3 = collision::ColliderBuilder::new()
            .offset(nalgebra::Vector2::new(0.0, 0.1))
            .bounds(nalgebra::Vector2::new(0.2, 0.1))
            .collision_groups(&[crate::systems::physics::collision_groups::ONE_WAY])
            .build(&mut collision_world, tile3);

        world
            .write_storage::<Collider>()
            .insert(player, player_collider)
            .unwrap();

        world
            .write_storage::<Collider>()
            .insert(tile1, tile_collider_1)
            .unwrap();
        world
            .write_storage::<Collider>()
            .insert(tile2, tile_collider_2)
            .unwrap();
        world
            .write_storage::<Collider>()
            .insert(tile3, tile_collider_3)
            .unwrap();
    }
}
