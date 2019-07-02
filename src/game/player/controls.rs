use crate::game::PlayerController;
use crate::components::*;

#[derive(Clone)]
pub enum InputDevice{
    Keyboard(),
    Gamepad(gilrs::GamepadId)
}

#[derive(Clone)]
pub enum PlayerEvent{
    Left,
    Right,
    Shoot,
    Crouch,
    Jump
}

#[derive(Clone)]
pub struct PlayerEvents(pub Vec<PlayerEvent>);


pub struct ControlsSystem;

impl<'a> specs::System<'a> for ControlsSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::WriteStorage<'a, PlayerController>,
    );
    fn run(&mut self, (input, mut controllers): Self::SystemData) {
        use specs::Join;
        for controller in (&mut controllers).join(){
            match controller.input_device {
                InputDevice::Keyboard() => {
                    update_controls_keyboard(controller, &input);
                },
                InputDevice::Gamepad(id) => {
                    update_controls_gamepad(controller, &input, id);
                },
            }
        }
    }
}

fn update_controls_keyboard(player: &mut PlayerController, input: &Input){
    use input::InputType;
    use glium::glutin::VirtualKeyCode as Key;

    if input.key_pressed(&InputType::KeyboardButton(Key::D)) {
        player.events.0.push(PlayerEvent::Right);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::A)) {
        player.events.0.push(PlayerEvent::Left);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::W)) {
        player.events.0.push(PlayerEvent::Jump);
    }
}

fn update_controls_gamepad(player: &mut PlayerController, input: &Input, id: gilrs::GamepadId){
    use input::InputType;
    use gilrs::Button;

    if input.key_pressed(&InputType::ControllerButton(Button::DPadRight, id)) {
        player.events.0.push(PlayerEvent::Right);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::DPadLeft, id)) {
        player.events.0.push(PlayerEvent::Left);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::DPadUp, id)) {
        player.events.0.push(PlayerEvent::Jump);
    }
}
