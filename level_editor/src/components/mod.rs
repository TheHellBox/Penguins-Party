pub mod camera;
pub mod child;
pub mod drawable;
pub mod input;
pub mod tile;
pub mod tile_list;
pub mod transform;

pub type Transform = transform::Transform;
pub type Drawable = drawable::Drawable;
pub type Camera = camera::Camera;
pub type Input = input::Input;
pub type Child = child::Child;
pub type TileList = tile_list::TileList;
pub type Tile = tile::Tile;

pub fn register_default(world: &mut specs::World) {
    world.register::<Drawable>();
    world.register::<Transform>();
    world.register::<Child>();
    world.register::<Tile>();
    add_default_resources(world);
}

fn add_default_resources(world: &mut specs::World) {
    world.add_resource(Input {
        ..Default::default()
    });
    world.add_resource(tile_list::TileList(
        crate::loaders::tile_loader::scan_tiles(),
    ));
    world.add_resource(Camera::new((1024.0, 768.0)));
}

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
        .with(child::ChildController, "Child Controller", &[])
        .with(input::InputCleaningSystem, "Input Cleaning System", &[])
}
