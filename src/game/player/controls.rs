use crate::components::*;
use crate::game::PlayerController;

#[derive(Clone)]
pub enum InputDevice {
    Keyboard(),
    Gamepad(gilrs::GamepadId),
}

#[derive(Clone)]
pub enum PlayerEvent {
    Left,
    Right,
    Shoot,
    Pick,
    Crouch,
    Jump,
}

pub type PlayerEvents = Vec<PlayerEvent>;

pub struct ControlsSystem;

impl<'a> specs::System<'a> for ControlsSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::WriteStorage<'a, PlayerController>,
    );
    fn run(&mut self, (input, mut controllers): Self::SystemData) {
        use specs::Join;
        for controller in (&mut controllers).join() {
            match controller.input_device {
                InputDevice::Keyboard() => {
                    update_controls_keyboard(controller, &input);
                }
                InputDevice::Gamepad(id) => {
                    update_controls_gamepad(controller, &input, id);
                }
            }
        }
    }
}

fn update_controls_keyboard(player: &mut PlayerController, input: &Input) {
    use glium::glutin::VirtualKeyCode as Key;
    use input::InputType;

    if input.key_pressed(&InputType::KeyboardButton(Key::D)) {
        player.events.push(PlayerEvent::Right);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::A)) {
        player.events.push(PlayerEvent::Left);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::S)) {
        player.events.push(PlayerEvent::Crouch);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::Space)) {
        player.events.push(PlayerEvent::Jump);
    }
    if input.key_pressed(&InputType::KeyboardButton(Key::E)) {
        player.events.push(PlayerEvent::Shoot);
    }
}

fn update_controls_gamepad(player: &mut PlayerController, input: &Input, id: gilrs::GamepadId) {
    use gilrs::Button;
    use input::InputType;

    if input.key_pressed(&InputType::ControllerButton(Button::DPadRight, id)) {
        player.events.push(PlayerEvent::Right);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::DPadLeft, id)) {
        player.events.push(PlayerEvent::Left);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::DPadDown, id)) {
        player.events.push(PlayerEvent::Crouch);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::South, id)) {
        player.events.push(PlayerEvent::Jump);
    }
    if input.key_pressed(&InputType::ControllerButton(Button::West, id)) {
        player.events.push(PlayerEvent::Shoot);
    }
}
