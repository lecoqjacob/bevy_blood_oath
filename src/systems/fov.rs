use bracket_pathfinding::prelude::field_of_view_set;

use crate::prelude::*;

pub fn fov(mut map: ResMut<Map>, mut views: Query<(&PointC, &mut FieldOfView, Option<&Player>)>) {
    for (pos, mut fov, player) in views.iter_mut() {
        if fov.is_dirty {
            fov.is_dirty = false;
            fov.visible_tiles = field_of_view_set(pos.0, fov.radius, map.as_ref());

            if player.is_some() {
                fov.visible_tiles.iter().for_each(|pt| {
                    map.revealed_tiles.insert(*pt);
                });
            }
        };
    }
}
