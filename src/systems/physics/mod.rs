pub mod collision;
pub mod collision_groups;

use crate::components::*;

pub struct GravitySystem;

impl<'a> specs::System<'a> for GravitySystem {
    type SystemData = (
        specs::Read<'a, GameState>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Physics>,
    );
    fn run(&mut self, (game_state, mut transforms, mut physic_objects): Self::SystemData) {
        use specs::Join;
        for (physics, transform) in (&mut physic_objects, &mut transforms).join() {
            let mut gravity = physics.gravity;
            if physics.on_ground {
                gravity = na::zero();
                physics.on_ground = false;
            }
            physics.force.y += gravity.y * game_state.frame_time_elapsed;
            transform.add_vector(physics.force * game_state.frame_time_elapsed);
            transform.physics_velocity = physics.force;
        }
    }
}
