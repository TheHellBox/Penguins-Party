use crate::components::Transform;
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;
use std::collections::HashMap;
#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Child {
    pub parent: specs::Entity,
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
                let parent_transform = transforms.get(child.parent).unwrap().clone();
                parent_transforms.insert(child.parent, parent_transform);
            }
        }
        {
            for (child, mut transform) in (&childs, &mut transforms).join() {
                // NOTE: It's better to use local and global transforms, but for now we'll just copy parent transform into child
                transform.position = parent_transforms
                    .get(&child.parent)
                    .unwrap()
                    .position
                    .clone();
            }
        }
    }
}
