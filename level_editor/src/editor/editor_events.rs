use crate::components::*;
use specs::world::Builder;

#[derive(Clone)]
pub enum EditorEvent {
    AddTile(String, na::Point2<f32>),
    RemoveTile(na::Point2<f32>),
    Export(String),
}

#[derive(Default, Clone)]
pub struct EditorEvents(pub Vec<EditorEvent>);

pub fn update_events(events: EditorEvents, world: &mut specs::World) {
    for event in &events.0 {
        match event {
            EditorEvent::AddTile(sprite, position) => {
                let _tile = world
                    .create_entity()
                    .with(Drawable {
                        sprite: sprite.clone(),
                        layer: 2,
                        ..Default::default()
                    })
                    .with(Transform {
                        position: position.clone(),
                        size: na::Vector2::repeat(0.2),
                        ..Default::default()
                    })
                    .with(tile::Tile)
                    .build();
            }
            EditorEvent::RemoveTile(position) => {
                use specs::Join;
                let mut remove = vec![];
                {
                    let transforms = world.read_storage::<Transform>();
                    let tiles = world.read_storage::<Tile>();
                    let entities = world.entities();
                    println!("{}", position);
                    for (entity, transform, _) in (&entities, &transforms, &tiles).join() {
                        if transform.position == *position {
                            remove.push(entity);
                        }
                    }
                }
                world.delete_entities(&remove).unwrap();
            }
            EditorEvent::Export(name) => {
                use specs::Join;
                let transforms = world.read_storage::<Transform>();
                let drawables = world.read_storage::<Drawable>();
                let tiles = world.read_storage::<Tile>();
                let (mut transforms_clone, mut drawables_clone) = (vec![], vec![]);
                for (transform, drawable, _) in (&transforms, &drawables, &tiles).join() {
                    transforms_clone.push(transform.clone());
                    drawables_clone.push(drawable.clone());
                }
                crate::editor::json_export::export(name.clone(), transforms_clone, drawables_clone)
            }
        }
    }
}
