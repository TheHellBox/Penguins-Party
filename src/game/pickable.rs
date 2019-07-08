use crate::components::*;
use crate::game::player::controls;
use crate::game::PlayerController;
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Default, Clone, Component)]
#[storage(VecStorage)]
pub struct Pickable {
    owner: Option<specs::Entity>,
}

impl Pickable {
    pub fn new() -> Self {
        Self { owner: None }
    }
    pub fn throw<'a>(
        &mut self,
        self_entity: specs::Entity,
        physics: &mut specs::WriteStorage<'a, Physics>,
        childs: &mut specs::WriteStorage<'a, Child>,
        transform: &Transform,
    ) {
        self.owner = None;
        let physics = physics.get_mut(self_entity);
        let child = childs.get_mut(self_entity);
        if let (Some(physics), Some(child)) = (physics, child) {
            physics.enabled = true;
            let direction = if transform.is_flip() { -1.0 } else { 1.0 };
            physics.apply_force(na::Vector2::new(5.0 * direction, 5.0));
            child.parent = self.owner;
        }
    }
    pub fn pick<'a>(
        &mut self,
        self_entity: specs::Entity,
        owner: specs::Entity,
        physics: &mut specs::WriteStorage<'a, Physics>,
        childs: &mut specs::WriteStorage<'a, Child>,
    ) {
        self.owner = Some(owner);

        let physics = physics.get_mut(self_entity);
        let child = childs.get_mut(self_entity);
        if let (Some(physics), Some(child)) = (physics, child) {
            physics.enabled = false;
            child.parent = self.owner;
        }
    }
}

pub struct PickableSystem;

impl<'a> specs::System<'a> for PickableSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::ReadStorage<'a, PlayerController>,
        specs::ReadStorage<'a, Transform>,
        specs::WriteStorage<'a, Physics>,
        specs::WriteStorage<'a, Child>,
        specs::WriteStorage<'a, Pickable>,
    );
    fn run(
        &mut self,
        (entities, players, transforms, mut physics, mut childs, mut pickables): Self::SystemData,
    ) {
        use specs::Join;
        for (player_entity, player, transform) in (&entities, &players, &transforms).join() {
            for event in &player.events {
                match event {
                    controls::PlayerEvent::Pick => {
                        use na::distance;
                        for (pickable_entity, pickable, pickable_transform) in
                            (&entities, &mut pickables, &transforms).join()
                        {
                            if pickable.owner.is_none() {
                                if distance(&transform.position, &pickable_transform.position) < 1.0
                                {
                                    pickable.pick(
                                        pickable_entity,
                                        player_entity,
                                        &mut physics,
                                        &mut childs,
                                    );
                                }
                            } else {
                                if pickable.owner.unwrap() == player_entity {
                                    pickable.throw(
                                        pickable_entity,
                                        &mut physics,
                                        &mut childs,
                                        pickable_transform,
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
