pub mod collision_groups;

use crate::components::*;

pub struct PhysicsSystem;

impl<'a> specs::System<'a> for PhysicsSystem {
    type SystemData = (
        specs::Read<'a, GameState>,
        specs::WriteExpect<'a, collision::CollisionWorld>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Physics>,
        specs::ReadStorage<'a, Collider>,
    );
    fn run(
        &mut self,
        (game_state, mut collision_world, mut transforms, mut physic_objects, colliders): Self::SystemData,
    ) {
        use specs::Join;
        for (physic_object, transform) in (&mut physic_objects, &mut transforms).join() {
            transform.add_vector(physic_object.gravity * game_state.frame_time_elapsed);
        }
        for (collider, transform) in (&colliders, &transforms).join() {
            collision_world.set_position(
                collider.handle,
                na::Isometry2::new(transform.position.coords.xy() + collider.offset, na::zero()),
            )
        }
        collision_world.update();
        for (collider_a, transform, physic_object) in
            (&colliders, &mut transforms, &mut physic_objects).join()
        {
            for collider_b in colliders.join() {
                if collider_a.handle != collider_b.handle {
                    if let Some((handle_a, handle_b, _, manifold)) =
                        collision_world.contact_pair(collider_a.handle, collider_b.handle, true)
                    {
                        for tracked_contact in manifold.deepest_contact() {
                            let contact = &tracked_contact.contact;
                            let normal = contact.normal.as_ref();
                            let penetration = normal * (contact.depth + 0.0001) * 0.5;
                            let vector = na::Vector3::new(penetration.x, penetration.y, 0.0);
                            transform.position -= vector;
                            if *normal == na::Vector2::new(0.0, -1.0) {
                                physic_object.on_ground = true;
                            }
                        }
                    }
                }
            }
        }
        for transform in (&mut transforms).join() {
            transform.velocity = na::Vector2::repeat(0.0);
        }
    }
}
