pub mod animation;
pub mod physics;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(physics::PhysicsSystem, "Physics System", &[])
        .with(animation::AnimationSystem, "Animation System", &[])
}
