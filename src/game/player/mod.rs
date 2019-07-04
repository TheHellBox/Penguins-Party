pub mod controls;
pub mod player_spawner;
pub mod player_killer;

use crate::components::*;

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct PlayerController{
    pub events: controls::PlayerEvents,
    pub input_device: controls::InputDevice,
    pub dead: bool,
    jump_started: std::time::Instant
}

pub struct PlayerControllerSystem;

impl PlayerController {
    pub fn new(input_device: controls::InputDevice) -> Self{
        Self{
            events: controls::PlayerEvents(vec![]),
            input_device: input_device,
            jump_started: std::time::Instant::now(),
            dead: false
        }
    }
    pub fn die(&mut self){
        self.dead = true;
    }
    pub fn alive(&self) -> bool{
        !self.dead
    }
}

impl<'a> specs::System<'a> for PlayerControllerSystem {
    type SystemData = (
        specs::Read<'a, GameState>,
        specs::WriteStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, AnimationController>,
        specs::WriteStorage<'a, Physics>,
    );
    fn run(
        &mut self,
        (game_state, mut players, mut transforms, mut animations, mut physics_objects): Self::SystemData,
    ) {
        use specs::Join;

        for (controller, transform, animation, physics) in
            (&mut players, &mut transforms, &mut animations, &mut physics_objects).join()
        {
            if !controller.alive(){
                continue
            }
            let mut velocity = na::Vector2::repeat(0.0);

            for event in &controller.events.0{
                match event {
                    controls::PlayerEvent::Left => {
                        velocity -= na::Vector2::new(4.0, 0.0);
                    },
                    controls::PlayerEvent::Right => {
                        velocity += na::Vector2::new(4.0, 0.0);
                    },
                    controls::PlayerEvent::Jump => {
                        let time = controller.jump_started.elapsed().as_millis();
                        if physics.on_ground && time > 100{
                            physics.apply_force(na::Vector2::new(0.0, 15.0) * game_state.frame_time_elapsed);
                            controller.jump_started = std::time::Instant::now();
                        }
                    },
                    controls::PlayerEvent::Crouch => {},
                    controls::PlayerEvent::Shoot => {},
                }
            }
            controller.events.0.clear();

            if velocity.x > 0.0 {
                transform.rotation = na::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
            } else if velocity.x < 0.0 {
                transform.rotation =
                    na::UnitQuaternion::from_euler_angles(0.0, std::f32::consts::PI, 0.0);
            }

            if velocity.x != 0.0 {
                animation.running = true;
            } else {
                animation.running = false;
                animation.current_frame = 0;
            }
            transform.add_vector(velocity * game_state.frame_time_elapsed);

            if transform.physics_velocity.y > 0.0 {
                physics.gravity.y = -1.5;
            } else {
                physics.gravity.y = -2.5;
            }

            if transform.position.y < -5.0{
                physics.gravity.y = 0.0;
                controller.die();
            }
        }
    }
}

pub fn spawn_player(world: &mut specs::World, position: na::Point2<f32>, input_device: controls::InputDevice) -> specs::Entity {
    use crate::components::*;
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
            .build(&mut collision_world, player);

        world
            .write_storage::<Collider>()
            .insert(player, player_collider)
            .unwrap();
    }
    player
}
