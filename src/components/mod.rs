pub mod animation_controller;
pub mod camera;
pub mod collision;
pub mod drawable;
pub mod input;
pub mod physics;
pub mod transform;

pub type Input = input::Input;
pub type Camera = camera::Camera;

pub type Drawable = drawable::Drawable;
pub type Transform = transform::Transform;
pub type Physics = physics::Physics;
pub type Collider = collision::Collider;
pub type AnimationController = animation_controller::AnimationController;

pub fn register_default(world: &mut specs::World) {
    world.register::<Drawable>();
    world.register::<Transform>();
    world.register::<Physics>();
    world.register::<Collider>();
    world.register::<AnimationController>();
    add_default_resources(world);
}

fn add_default_resources(world: &mut specs::World) {
    world.add_resource(Input {
        ..Default::default()
    });
    world.add_resource(Camera::new((1024.0, 768.0)));
    // Add collision
    world.add_resource(collision::init_collision_world());
}
