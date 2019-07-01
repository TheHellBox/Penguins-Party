use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component, Default)]
#[storage(VecStorage)]
pub struct AnimationController {
    pub frame_update_time: u32,
    pub frames_amount: u8,
    pub current_frame: u8,
    pub current_row: u8,
    pub rows: u8,
    pub next_frame_time: u32,
    pub running: bool,
}

impl AnimationController {
    pub fn new(frames_amount: u8) -> Self {
        Self {
            frames_amount,
            frame_update_time: 100,
            rows: 1,
            running: true,
            current_row: 1,
            ..Default::default()
        }
    }
    pub fn frame_update_speed(mut self, frame_update_time: u32) -> Self {
        self.frame_update_time = frame_update_time;
        self
    }
    pub fn _rows(mut self, rows: u8) -> Self {
        self.rows = rows;
        self
    }
}
