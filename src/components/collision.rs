use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

pub type CollisionWorld = ncollide2d::world::CollisionWorld<f32, specs::Entity>;

pub struct ColliderBuilder {
    pub offset: nalgebra::Vector2<f32>,
    pub shape: ncollide2d::shape::Cuboid<f32>,
    pub collision_group: ncollide2d::world::CollisionGroups,
}

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub handle: ncollide2d::world::CollisionObjectHandle,
    pub offset: nalgebra::Vector2<f32>
}

pub fn init_collision_world() -> CollisionWorld {
    CollisionWorld::new(0.02)
}

#[allow(dead_code)]
impl ColliderBuilder {
    pub fn new() -> Self {
        Self {
            offset: nalgebra::Vector2::repeat(0.0),
            shape: ncollide2d::shape::Cuboid::new(nalgebra::Vector2::repeat(1.0)),
            collision_group: ncollide2d::world::CollisionGroups::new(),
        }
    }
    pub fn offset(mut self, offset: nalgebra::Vector2<f32>) -> Self {
        self.offset = offset;
        self
    }
    pub fn bounds(mut self, bounds: nalgebra::Vector2<f32>) -> Self {
        self.shape = ncollide2d::shape::Cuboid::new(bounds);
        self
    }
    pub fn collision_groups(mut self, group_id: &[usize]) -> Self {
        let mut group = ncollide2d::world::CollisionGroups::new();
        group.set_membership(group_id);
        self.collision_group = group;
        self
    }
    pub fn build(self, collision_world: &mut CollisionWorld, entity: specs::Entity) -> Collider {
        let object = collision_world.add(
            nalgebra::Isometry2::new(self.offset, nalgebra::zero()),
            ncollide2d::shape::ShapeHandle::new(self.shape),
            self.collision_group,
            ncollide2d::world::GeometricQueryType::Contacts(0.0, 0.0),
            entity,
        );
        Collider {
            handle: object.handle(),
            offset: self.offset
        }
    }
}