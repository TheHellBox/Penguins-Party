use crate::components::Vector2;

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

pub type CollisionWorld = ncollide2d::world::CollisionWorld<f32, specs::Entity>;

pub struct ColliderBuilder {
    pub offset: Vector2,
    pub shape: ncollide2d::shape::Cuboid<f32>,
    pub collision_group: ncollide2d::world::CollisionGroups,
}

#[derive(Clone)]
pub struct Collider {
    pub enabled: bool,
    pub handle: ncollide2d::world::CollisionObjectHandle,
    pub offset: Vector2,
    // Vector 2 in this case is normal
    pub collides_with: Vec<(specs::Entity, Vector2)>,
}

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Colliders(pub Vec<Collider>);

pub fn init_collision_world() -> CollisionWorld {
    CollisionWorld::new(0.02)
}

#[allow(dead_code)]
impl ColliderBuilder {
    pub fn new() -> Self {
        Self {
            offset: na::zero(),
            shape: ncollide2d::shape::Cuboid::new(na::Vector2::repeat(1.0)),
            collision_group: ncollide2d::world::CollisionGroups::new(),
        }
    }
    pub fn offset(mut self, offset: Vector2) -> Self {
        self.offset = offset;
        self
    }
    pub fn bounds(mut self, bounds: Vector2) -> Self {
        self.shape = ncollide2d::shape::Cuboid::new(bounds);
        self
    }
    pub fn membership(mut self, group_id: &[usize]) -> Self {
        self.collision_group.set_membership(group_id);
        self
    }
    pub fn whitelist(mut self, group_id: &[usize]) -> Self {
        self.collision_group.set_whitelist(group_id);
        self
    }
    pub fn blacklist(mut self, group_id: &[usize]) -> Self {
        self.collision_group.set_blacklist(group_id);
        self
    }
    pub fn build(self, collision_world: &mut CollisionWorld, entity: specs::Entity) -> Collider {
        println!("{:?}", self.collision_group);
        let object = collision_world.add(
            na::Isometry2::new(self.offset, na::zero()),
            ncollide2d::shape::ShapeHandle::new(self.shape.clone()),
            self.collision_group,
            ncollide2d::world::GeometricQueryType::Contacts(0.0001, 0.0),
            entity,
        );
        Collider {
            enabled: true,
            handle: object.handle(),
            offset: self.offset,
            collides_with: vec![],
        }
    }
}
