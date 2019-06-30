use crate::components::{AnimationController, Drawable};

pub struct AnimationSystem;

impl<'a> specs::System<'a> for AnimationSystem {
    type SystemData = (
        specs::WriteStorage<'a, AnimationController>,
        specs::WriteStorage<'a, Drawable>,
    );
    fn run(&mut self, (mut controllers, mut drawables): Self::SystemData) {
        use specs::Join;
        use std::time::SystemTime;

        for (controller, drawable) in (&mut controllers, &mut drawables).join() {
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u32;
            if current_time > controller.next_frame_time && controller.running {
                let x_step = 1.0 / controller.frames_amount as f32;
                let y_step = 1.0 / controller.rows as f32;
                drawable.uv_bounds = [x_step, y_step];
                if controller.current_frame < controller.frames_amount {
                    drawable.uv_offset[0] = x_step * controller.current_frame as f32;
                    controller.current_frame += 1;
                } else if controller.current_row < controller.rows {
                    drawable.uv_offset[1] = y_step * controller.current_frame as f32;
                    controller.current_row += 1;
                    controller.current_frame = 0;
                } else {
                    drawable.uv_offset = [0.0, 0.0];
                    controller.current_frame = 0;
                    controller.current_row = 1;
                }
                controller.next_frame_time = current_time + controller.frame_update_time;
            }
        }
    }
}
