use crate::components::{Point3, Transform, UnitQuaternion};
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;
use std::collections::HashMap;
#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Child {
    pub local_position: Point3,
    pub local_rotation: UnitQuaternion,
    pub parent: Option<specs::Entity>,
}

pub struct ChildController;

impl<'a> specs::System<'a> for ChildController {
    type SystemData = (
        specs::ReadStorage<'a, Child>,
        specs::WriteStorage<'a, Transform>,
    );
    fn run(&mut self, (childs, mut transforms): Self::SystemData) {
        use specs::Join;
        let mut parent_transforms = HashMap::new();
        {
            for (child, transform) in (&childs, &transforms).join() {
                if let Some(parent) = child.parent {
                    let parent_transform = transforms.get(parent).unwrap().clone();
                    parent_transforms.insert(parent, parent_transform);
                }
            }
        }
        {
            for (child, mut transform) in (&childs, &mut transforms).join() {
                if let Some(parent) = child.parent {
                    let parent_transform = parent_transforms.get(&parent).unwrap();
                    let isometry = na::Isometry3::from_parts(
                        na::Translation3::from(parent_transform.position.coords),
                        parent_transform.rotation,
                    );
                    let isometry2 = na::Isometry3::from_parts(
                        na::Translation3::from(child.local_position.coords),
                        child.local_rotation,
                    );
                    let result = isometry * isometry2;
                    transform.position.coords = result.translation.vector;
                    transform.rotation = result.rotation;
                }
            }
        }
    }
}
