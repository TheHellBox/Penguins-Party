use crate::components::*;

pub fn update(world: &mut specs::World) {
    use crate::game::player;
    let events = world.read_resource::<Input>().gilrs_events.clone();

    for (event, id) in &events {
        match event {
            gilrs::EventType::Connected => {
                player::spawn_player(
                    world,
                    na::Point2::new(0.0, 2.0),
                    player::controls::InputDevice::Gamepad(*id),
                );
            }
            _ => {}
        }
    }
}
