use crate::systems::physics::collision_groups::*;
use crate::components::collision::*;
use crate::components::{Transform};

pub struct OneWaySystem;

impl<'a> specs::System<'a> for OneWaySystem {
    type SystemData = (
        specs::ReadExpect<'a, CollisionWorld>,
        specs::WriteStorage<'a, Collider>,
        specs::ReadStorage<'a, Transform>,
    );
    fn run(&mut self, (collision_world, mut colliders, transforms): Self::SystemData) {
        use specs::Join;
        for (collider, transform) in (&mut colliders, &transforms).join(){
            let object = collision_world.collision_object(collider.handle).unwrap();
            let groups = object.collision_groups();
            // An ungly way to detect right collider, but it works
            if groups.is_member_of(PLAYER) && groups.is_group_whitelisted(ONE_WAY){
                collider.enabled = transform.physics_velocity.y <= 0.0;
            }
        }
    }
}
