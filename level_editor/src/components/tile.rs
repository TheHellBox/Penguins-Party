use specs::Component;
use specs::VecStorage;
use specs_derive::Component;
#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Tile;
