use crate::components::*;
use crate::game::PlayerController;

pub struct PlayerKillerSystem;

impl<'a> specs::System<'a> for PlayerKillerSystem {
    type SystemData = (
        specs::Entities<'a>,
        specs::WriteStorage<'a, PlayerController>,
        specs::ReadStorage<'a, Collider>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, Drawable>,
        specs::WriteStorage<'a, Physics>,
    );
    fn run(
        &mut self,
        (entities, mut players, colliders, mut transforms, mut drawables, mut physics): Self::SystemData,
    ) {
        use specs::Join;
        let mut try_kill = vec![];
        for (entity, player, collider, transform, drawable) in (
            &entities,
            &mut players,
            &colliders,
            &mut transforms,
            &mut drawables,
        )
            .join()
        {
            if player.dead && drawable.enabled {
                drawable.enabled = false;
                transform.set_position(na::Point2::new(999.0, 999.0));
            }
            if !player.dead && !drawable.enabled {
                drawable.enabled = true;
            }
            if transform.position.y < -5.0 {
                player.die();
            }
            try_kill = collider.collides_with.iter().map(|a| (entity, a)).collect();
        }
        for (self_entity, (entity, normal)) in try_kill {
            let player = players.get_mut(*entity);
            let self_physics = physics.get_mut(self_entity);
            if let (Some(player), Some(physics)) = (player, self_physics) {
                if *normal == na::Vector2::new(0.0, -1.0) {
                    player.die();
                    physics.apply_force(na::Vector2::new(0.0, 2.0));
                }
            }
        }
    }
}
