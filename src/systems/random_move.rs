use crate::prelude::*;

pub fn random_move(
    map: Res<Map>,
    rng: Res<RandomNumbers>,
    mut movers: Query<&mut Position, With<Colonist>>,
) {
    for mut pos in movers.iter_mut() {
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + pos.pt;

        let new_idx = map.get_current().point2d_to_index(destination);
        if map.get_current().in_bounds(destination) && !map.get_current().tiles[new_idx].blocked {
            pos.pt = destination;
        }
    }
}
