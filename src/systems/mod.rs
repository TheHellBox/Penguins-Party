pub mod animation;
pub mod physics;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(physics::GravitySystem, "Gravity System", &[])
        .with(physics::collision::CollisionSystem, "Collision System", &[])
        .with(animation::AnimationSystem, "Animation System", &[])
}
