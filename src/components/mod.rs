pub mod animation_controller;
pub mod camera;
pub mod collision;
pub mod drawable;
pub mod game_state;
pub mod input;
pub mod physics;
pub mod transform;

pub type Vector2 = na::Vector2<f32>;
pub type Point3 = na::Point3<f32>;

pub type Input = input::Input;
pub type Camera = camera::Camera;

pub type Drawable = drawable::Drawable;
pub type Transform = transform::Transform;
pub type Physics = physics::Physics;
pub type Colliders = collision::Colliders;
pub type AnimationController = animation_controller::AnimationController;
pub type GameState = game_state::GameState;

pub fn register_default(world: &mut specs::World) {
    world.register::<Drawable>();
    world.register::<Transform>();
    world.register::<Physics>();
    world.register::<Colliders>();
    world.register::<Camera>();
    world.register::<AnimationController>();
    add_default_resources(world);
}

fn add_default_resources(world: &mut specs::World) {
    world.add_resource(Input {
        ..Default::default()
    });
    world.add_resource(GameState {
        ..Default::default()
    });
    // Add collision
    world.add_resource(collision::init_collision_world());
}

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder.with(input::InputCleaningSystem, "Input Cleaning System", &[])
}
