use crate::components::collision::*;
use crate::components::Transform;
use crate::game::PlayerController;
use crate::systems::physics::collision_groups::*;

pub struct OneWaySystem;

impl<'a> specs::System<'a> for OneWaySystem {
    type SystemData = (
        specs::ReadExpect<'a, CollisionWorld>,
        specs::ReadStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Colliders>,
        specs::ReadStorage<'a, Transform>,
    );
    fn run(&mut self, (collision_world, players, mut colliders, transforms): Self::SystemData) {
        use specs::Join;
        for (_player, collider, transform) in (&players, &mut colliders, &transforms).join() {
            for collider in &mut collider.0 {
                let object = collision_world.collision_object(collider.handle).unwrap();
                let groups = object.collision_groups();
                // An ungly way to detect right collider, but it works
                if groups.is_member_of(PLAYER)
                    && groups.is_group_whitelisted(ONE_WAY)
                    && groups.is_group_blacklisted(TILE)
                {
                    collider.enabled = transform.physics_velocity.y <= 0.0;
                }
            }
        }
    }
}
