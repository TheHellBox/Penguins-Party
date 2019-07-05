use crate::components::*;
use crate::game::PlayerController;

pub struct PlayerAnimationSystem;

impl<'a> specs::System<'a> for PlayerAnimationSystem {
    type SystemData = (
        specs::ReadStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, AnimationController>,
    );
    fn run(&mut self, (players, mut transforms, mut animations): Self::SystemData) {
        use specs::Join;
        for (player, transform, animation) in (&players, &mut transforms, &mut animations).join() {
            if player.velocity.x > 0.0 {
                transform.rotation = na::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
            } else if player.velocity.x < 0.0 {
                transform.rotation =
                    na::UnitQuaternion::from_euler_angles(0.0, std::f32::consts::PI, 0.0);
            }

            if player.velocity.x == 0.0 {
                animation.current_frame = 0;
            }
        }
    }
}
