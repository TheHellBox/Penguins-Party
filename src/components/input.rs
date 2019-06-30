use std::collections::HashMap;

#[derive(Default)]
pub struct Input {
    pub events: Vec<glium::glutin::WindowEvent>,
    pub keys_state: HashMap<glium::glutin::VirtualKeyCode, bool>,
    pub mouse_buttons_state: HashMap<glium::glutin::MouseButton, bool>,
}

#[allow(dead_code)]
impl Input {
    pub fn key_pressed(&self, keycode: &glium::glutin::VirtualKeyCode) -> bool {
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
