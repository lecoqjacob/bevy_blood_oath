use crate::prelude::*;

pub fn movement(
    mut move_events: EventReader<WantsToMove>,
    mut query: Query<(&mut FieldOfView, Option<&Player>)>,
    (map, mut camera): (Res<Map>, ResMut<GameCamera>),
    mut posisitons: Query<&mut Position>,
) {
    for &WantsToMove {
        entity,
        destination,
    } in move_events.iter()
    {
        if map.get_current().in_bounds(destination) {
            // commands.entity(entity).insert(Position(destination));
            let mut pos = posisitons.get_mut(entity).unwrap();
            pos.pt = destination;

            if let Ok((mut fov, player)) = query.get_mut(entity) {
                // In Bevy, we don't need to test for Result<FieldOfView>, because the entity, if found,
                // will have a FieldOfView component, due to the query definition.
                // commands.entity(entity).insert(fov.clone_dirty());
                fov.is_dirty = true;

                if player.is_some() {
                    camera.on_player_move(destination);
                    // fov.visible_tiles.iter().for_each(|pos| {
                    //     map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    // });
                }
            }
        }
    }
}
