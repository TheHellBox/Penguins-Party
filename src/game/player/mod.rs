pub mod controls;
pub mod player_animation;
pub mod player_killer;
pub mod player_spawner;

use crate::components::*;

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct PlayerController {
    pub events: controls::PlayerEvents,
    pub input_device: controls::InputDevice,
    pub dead: bool,
    pub velocity: Vector2,
    jump_started: std::time::Instant,
}

pub struct PlayerControllerSystem;

impl PlayerController {
    pub fn new(input_device: controls::InputDevice) -> Self {
        Self {
            events: vec![],
            input_device: input_device,
            jump_started: std::time::Instant::now(),
            velocity: na::zero(),
            dead: false,
        }
    }
    pub fn die(&mut self) {
        self.dead = true;
    }
    pub fn alive(&self) -> bool {
        !self.dead
    }
    pub fn get_events(&mut self) -> controls::PlayerEvents {
        let events = self.events.clone();
        self.events.clear();
        events
    }
}

impl<'a> specs::System<'a> for PlayerControllerSystem {
    type SystemData = (
        specs::Read<'a, GameState>,
        specs::WriteStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Physics>,
    );
    fn run(
        &mut self,
        (game_state, mut players, mut transforms, mut physics_objects): Self::SystemData,
    ) {
        use specs::Join;

        for (controller, transform, physics) in
            (&mut players, &mut transforms, &mut physics_objects).join()
        {
            if !controller.alive() {
                continue;
            }
            let mut velocity = na::Vector2::repeat(0.0);

            for event in controller.get_events() {
                match event {
                    controls::PlayerEvent::Left => {
                        if !physics.hit_left_wall {
                            velocity -= na::Vector2::new(4.0, 0.0);
                        }
                    }
                    controls::PlayerEvent::Right => {
                        if !physics.hit_right_wall {
                            velocity += na::Vector2::new(4.0, 0.0);
                        }
                    }
                    controls::PlayerEvent::Jump => {
                        let time = controller.jump_started.elapsed().as_millis();
                        if physics.on_ground && time > 100 {
                            physics.apply_force(na::Vector2::new(0.0, 5.0));
                            controller.jump_started = std::time::Instant::now();
                        }
                    }
                    controls::PlayerEvent::Crouch => {}
                    controls::PlayerEvent::Shoot => {}
                }
            }
            controller.velocity = velocity;

            transform.add_vector(velocity * game_state.frame_time_elapsed);

            if transform.physics_velocity.y > 0.0 {
                physics.gravity.y = -5.0;
            } else {
                physics.gravity.y = -9.0;
            }
        }
    }
}

pub fn spawn_player(
    world: &mut specs::World,
    position: na::Point2<f32>,
    input_device: controls::InputDevice,
) -> specs::Entity {
    use crate::components::*;
    use crate::systems::physics::collision_groups::*;
    use specs::world::Builder;

    let player = world
        .create_entity()
        .with(Drawable::new(String::from("penguin")))
        .with(Transform {
            position: na::Point3::new(position.x, position.y, 0.0),
            size: na::Vector2::new(0.3, 0.3),
            ..Default::default()
        })
        .with(PlayerController::new(input_device))
        .with(Physics::new())
        .with(AnimationController::new(16).frame_update_speed(50))
        .build();

    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        let player_collider = collision::ColliderBuilder::new()
            .bounds(na::Vector2::new(0.2, 0.3))
            .membership(&[PLAYER])
            .blacklist(&[ONE_WAY, PLAYER])
            .build(&mut collision_world, player);

        let player_platform_collider = collision::ColliderBuilder::new()
            .bounds(na::Vector2::new(0.2, 0.01))
            .offset(na::Vector2::new(0.0, -0.3))
            .membership(&[PLAYER])
            .blacklist(&[PLAYER])
            .whitelist(&[ONE_WAY])
            .build(&mut collision_world, player);

        world
            .write_storage::<Collider>()
            .insert(player, player_collider)
            .unwrap();
        world
            .write_storage::<Collider>()
            .insert(player, player_platform_collider)
            .unwrap();
    }
    player
}
