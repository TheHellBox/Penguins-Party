use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum InputType {
    KeyboardButton(glium::glutin::VirtualKeyCode),
    ControllerButton(gilrs::Button, gilrs::GamepadId),
}

#[derive(Default)]
pub struct Input {
    pub window_events: Vec<glium::glutin::WindowEvent>,
    pub gilrs_events: Vec<(gilrs::EventType, gilrs::GamepadId)>,
    pub keys_state: HashMap<InputType, bool>,
    pub prev_keys_state: HashMap<InputType, bool>,
    pub mouse_buttons_state: HashMap<glium::glutin::MouseButton, bool>,
}

#[allow(dead_code)]
impl Input {
    pub fn key_hold(&self, keycode: &InputType) -> bool {
        if let Some(state) = self.keys_state.get(keycode) {
            state.clone()
        } else {
            false
        }
    }
    pub fn key_pressed(&self, keycode: &InputType) -> bool {
        if let (Some(state), Some(prev_state)) = (
            self.keys_state.get(keycode),
            self.prev_keys_state.get(keycode),
        ) {
            if state != prev_state {
                state.clone()
            } else {
                false
            }
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
