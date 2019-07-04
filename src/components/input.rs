use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum InputType {
    KeyboardButton(glium::glutin::VirtualKeyCode),
    ControllerButton(gilrs::Button, gilrs::GamepadId),
}

#[derive(Default)]
pub struct Input {
    pub window_events: Vec<glium::glutin::WindowEvent>,
    pub gilrs_events: Vec<(gilrs::EventType, gilrs::GamepadId)>,
    pub keys_state: HashMap<InputType, bool>,
    pub mouse_buttons_state: HashMap<glium::glutin::MouseButton, bool>,
}

#[allow(dead_code)]
impl Input {
    pub fn key_pressed(&self, keycode: &InputType) -> bool {
        if let Some(state) = self.keys_state.get(keycode) {
            state.clone()
        } else {
            false
        }
    }
    pub fn mouse_button_pressed(&self, button: &glium::glutin::MouseButton) -> bool {
        if let Some(state) = self.mouse_buttons_state.get(button) {
            state.clone()
        } else {
            false
        }
    }
}

pub struct InputCleaningSystem;

impl<'a> specs::System<'a> for InputCleaningSystem {
    type SystemData = specs::Write<'a, Input>;
    fn run(&mut self, mut input: Self::SystemData) {
        input.window_events.clear();
        input.gilrs_events.clear();
    }
}
