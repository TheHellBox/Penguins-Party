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
    pub fn throw(&mut self){
        self.owner = None;
    }
    pub fn pick(&mut self, owner: specs::Entity){
        self.owner = Some(owner);
    }
}

pub struct PickableSystem;

impl<'a> specs::System<'a> for PickableSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::ReadStorage<'a, PlayerController>,
        specs::ReadStorage<'a, Transform>,
        specs::WriteStorage<'a, Pickable>,
    );
    fn run(&mut self, (entities, players, transforms, mut pickables): Self::SystemData) {
        use specs::Join;
        for (player_entity, player, transform) in (&entities, &players, &transforms).join() {
            for event in &player.events {
                match event {
                    controls::PlayerEvent::Pick => {
                        use na::distance;
                        for (pickable, pickable_transform) in (&mut pickables, &transforms).join() {
                            if pickable.owner.is_none(){
                                if distance(&transform.position, &pickable_transform.position) < 1.0 {
                                    pickable.pick(player_entity);
                                }
                            }
                            else{
                                if pickable.owner.unwrap() == player_entity{
                                    pickable.throw();
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
