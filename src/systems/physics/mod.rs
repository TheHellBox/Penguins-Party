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

        for (handle_a, handle_b, _, contact_manifold) in collision_world.contact_pairs(true) {
            for tracked_contact in contact_manifold.deepest_contact() {
                let collision_object_a = collision_world.collision_object(handle_a).unwrap();
                let entity_a = collision_object_a.data();

                let collision_object_b = collision_world.collision_object(handle_b).unwrap();
                let entity_b = collision_object_b.data();

                let contact = &tracked_contact.contact;
                let normal = contact.normal.as_ref().clone();
                let vector = (contact.depth + 0.0001) * normal * 0.5;

                let physics_object_a = physic_objects.get_mut(*entity_a);
                let transform_a = transforms.get_mut(*entity_a);

                if let (Some(physics_object), Some(transform)) = (physics_object_a, transform_a) {
                    if normal == nalgebra::Vector2::new(0.0, -1.0) {
                        physics_object.on_ground = true;
                    } else {
                        physics_object.on_ground = false;
                    }
                    transform.position -= nalgebra::Vector3::new(vector.x, vector.y, 0.0);
                }
                let physics_object_b = physic_objects.get_mut(*entity_b);
                let transform_b = transforms.get_mut(*entity_b);

                if let (Some(physics_object), Some(transform)) = (physics_object_b, transform_b) {
                    if normal == nalgebra::Vector2::new(0.0, -1.0) {
                        physics_object.on_ground = true;
                    } else {
                        physics_object.on_ground = false;
                    }
                    transform.position += nalgebra::Vector3::new(vector.x, vector.y, 0.0);
                }
            }
        }
    }
}
