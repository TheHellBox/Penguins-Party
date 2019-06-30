pub mod collision_groups;

use crate::components::*;

pub struct PhysicsSystem;

impl<'a> specs::System<'a> for PhysicsSystem {
    type SystemData = (
        specs::WriteExpect<'a, collision::CollisionWorld>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Physics>,
        specs::ReadStorage<'a, Collider>,
    );
    fn run(
        &mut self,
        (mut collision_world, mut transforms, mut physic_objects, colliders): Self::SystemData,
    ) {
        use specs::Join;
        for (collider, transform) in (&colliders, &transforms).join() {
            collision_world.set_position(
                collider.handle,
                na::Isometry2::new(transform.position.coords.xy() + collider.offset, na::zero()),
            )
        }
        collision_world.update();
        for (physic_object, transform) in (&physic_objects, &mut transforms).join() {
            transform.add_vector(physic_object.gravity);
        }
        for (collider, transform) in (&colliders, &transforms).join() {
            collision_world.set_position(
                collider.handle,
                na::Isometry2::new(transform.position.coords.xy() + collider.offset, na::zero()),
            )
        }
        collision_world.update();
        for (handle_a, handle_b, _, contact_manifold) in collision_world.contact_pairs(true) {
            for contact in contact_manifold.contacts() {
                let collision_object_a = collision_world.collision_object(handle_a).unwrap();
                let collision_object_b = collision_world.collision_object(handle_b).unwrap();
                let entity_a = collision_object_a.data();

                let contact = &contact.contact;
                let normal = contact.normal.as_ref().clone();
                let vector = contact.depth * normal * 0.5;

                let physics_object_a = physic_objects.get_mut(*entity_a);
                let transform_a = transforms.get_mut(*entity_a);

                let _groups = collision_object_b.collision_groups();
                if let (Some(physics_object), Some(transform)) = (physics_object_a, transform_a) {
                    if normal == na::Vector2::new(0.0, -1.0) {
                        physics_object.on_ground = true;
                    } else {
                        physics_object.on_ground = false;
                    }
                    transform.position -= na::Vector3::new(vector.x, vector.y, 0.0);
                }
            }
        }
        for event in collision_world.contact_events() {
            use ncollide2d::events::ContactEvent::*;
            match event {
                Started(_a, _b) => {}
                Stopped(a, _b) => {
                    let physics_object = physic_objects
                        .get_mut(*collision_world.collision_object(*a).unwrap().data());
                    if let Some(physics_object) = physics_object {
                        physics_object.on_ground = false;
                    }
                }
            }
        }
        for transform in (&mut transforms).join() {
            transform.velocity = na::Vector2::repeat(0.0);
        }
    }
}
