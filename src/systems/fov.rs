use bracket_pathfinding::prelude::field_of_view_set;

use crate::prelude::*;

pub fn fov(
    mut map: ResMut<Map>,
    mut views: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
    mut colonist: Query<(&mut ColonistStatus, &Position), With<Colonist>>,
) {
    for (pos, mut fov, player) in views.iter_mut() {
        if fov.is_dirty {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.pt, fov.radius, map.get_current());

            if player.is_some() {
                let current_layer = map.get_current_mut();

                current_layer.clear_visible();
                fov.visible_tiles.iter().for_each(|pt| {
                    if current_layer.in_bounds(*pt) {
                        let idx = current_layer.point2d_to_index(*pt);
                        current_layer.revealed[idx] = true;
                        current_layer.visible[idx] = true;
                    }
                });

                for (mut status, pos) in colonist.iter_mut() {
                    if pos.layer == map.current_layer && fov.visible_tiles.contains(&pos.pt) {
                        // TODO: All the other possibilities including being dead
                        match *status {
                            ColonistStatus::Unknown => *status = ColonistStatus::Alive,
                            _ => {}
                        }
                    }
                }
            }
        };
    }
}
