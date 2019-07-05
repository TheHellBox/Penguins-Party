use crate::components::*;

pub struct CollisionSystem;

impl<'a> specs::System<'a> for CollisionSystem {
    type SystemData = (
        specs::WriteExpect<'a, collision::CollisionWorld>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Collider>,
        specs::WriteStorage<'a, Physics>,
    );
    fn run(
        &mut self,
        (mut collision_world, mut transforms, mut colliders, mut physic_objects): Self::SystemData,
    ) {
        use specs::Join;
        for (collider, transform) in (&mut colliders, &transforms).join() {
            collider.collides_with.clear();
            collision_world.set_position(
                collider.handle,
                na::Isometry2::new(transform.position.coords.xy() + collider.offset, na::zero()),
            )
        }
        collision_world.update();

        for (handle_a, handle_b, _, contact_manifold) in collision_world.contact_pairs(true) {
            let collision_object_a = collision_world.collision_object(handle_a).unwrap();
            let entity_a = collision_object_a.data();

            let collision_object_b = collision_world.collision_object(handle_b).unwrap();
            let entity_b = collision_object_b.data();

            for tracked_contact in contact_manifold.deepest_contact() {
                let contact = &tracked_contact.contact;
                let normal = contact.normal.as_ref().clone();
                let vector = (contact.depth + 0.0001) * (normal * 0.5);

                let collider_a = colliders.get_mut(*entity_a).unwrap();
                collider_a.collides_with.push((*entity_b, normal));
                let collider_b = colliders.get_mut(*entity_b).unwrap();
                collider_b.collides_with.push((*entity_a, -normal));
                let physics_object_a = physic_objects.get_mut(*entity_a);
                let transform_a = transforms.get_mut(*entity_a);
                if let (Some(physics_object), Some(transform)) = (physics_object_a, transform_a) {
                    if normal == nalgebra::Vector2::new(0.0, -1.0) {
                        physics_object.on_ground = true;
                    }
                    else{
                        physics_object.on_ground = false;
                    }
                    physics_object.force = na::zero();
                    transform.position -= nalgebra::Vector3::new(vector.x, vector.y, 0.0);
                }
                let physics_object_b = physic_objects.get_mut(*entity_b);
                let transform_b = transforms.get_mut(*entity_b);

                if let (Some(physics_object), Some(transform)) = (physics_object_b, transform_b) {
                    if normal == nalgebra::Vector2::new(0.0, 1.0) {
                        physics_object.on_ground = true;
                    }
                    else{
                        physics_object.on_ground = false;
                    }
                    physics_object.force = na::zero();
                    transform.position += nalgebra::Vector3::new(vector.x, vector.y, 0.0);
                }
            }
        }
        for event in collision_world.contact_events() {
            use ncollide2d::events::ContactEvent::*;
            match event {
                Started(_a, _b) => {}
                Stopped(a, b) => {
                    let physics_object = physic_objects
                        .get_mut(*collision_world.collision_object(*a).unwrap().data());
                    if let Some(physics_object) = physics_object {
                        physics_object.on_ground = false;
                    }
                    let physics_object = physic_objects
                        .get_mut(*collision_world.collision_object(*b).unwrap().data());
                    if let Some(physics_object) = physics_object {
                        physics_object.on_ground = false;
                    }
                }
            }
        }
    }
}
