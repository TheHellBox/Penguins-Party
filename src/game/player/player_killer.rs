pub fn update(world: &mut specs::World) {
    use specs::Join;
    let players = world.read_storage::<crate::game::PlayerController>();
    let mut drawables = world.write_storage::<crate::game::Drawable>();
    for (player, drawable) in (&players, &mut drawables).join() {
        if player.dead && drawable.enabled {
            drawable.enabled = false;
        }
        if !player.dead && !drawable.enabled {
            drawable.enabled = true;
        }
    }
}
