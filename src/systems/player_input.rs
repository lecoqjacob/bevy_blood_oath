use crate::prelude::*;
use std::collections::HashSet;

pub fn player_input_system(
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut player_query: Query<(&mut Position, &mut FieldOfView), (With<Player>, Without<Door>)>,
    doors: Query<(Entity, &Position), (With<Door>, Without<Player>)>,
    // mut attack_events: EventWriter<WantsToAttack>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut camera: ResMut<GameCamera>,
) {
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        let mut doors_to_delete = HashSet::new();

        let delta = match key {
            KeyCode::Left | KeyCode::Numpad4 | KeyCode::H => Point::new(-1, 0),
            KeyCode::Right | KeyCode::Numpad6 | KeyCode::L => Point::new(1, 0),
            KeyCode::Up | KeyCode::Numpad8 | KeyCode::K => Point::new(0, -1),
            KeyCode::Down | KeyCode::Numpad2 | KeyCode::J => Point::new(0, 1),

            // Diagonals
            KeyCode::Numpad7 | KeyCode::Y => Point::new(-1, -1),
            KeyCode::Numpad9 | KeyCode::U => Point::new(1, -1),
            KeyCode::Numpad3 | KeyCode::N => Point::new(1, 1),
            KeyCode::Numpad1 | KeyCode::B => Point::new(-1, 1),
            _ => Point::new(0, 0),
        };

        let (mut pos, mut player_fov) = player_query.single_mut();

        if delta.x != 0 || delta.y != 0 {
            let new_pos = pos.pt + delta;
            let new_idx = map.get_current().point2d_to_index(new_pos);

            if !map.get_current().tiles[new_idx].blocked {
                pos.pt = new_pos;
                player_fov.is_dirty = true;
                camera.on_player_move(pos.pt)
            } else if map.get_current().is_door[new_idx] {
                map.get_current_mut().open_door(new_idx);
                doors_to_delete.insert(map.get_current().index_to_point2d(new_idx));
            }

            if !doors_to_delete.is_empty() {
                doors.iter().for_each(|(entity, pos)| {
                    if pos.layer == map.current_layer && doors_to_delete.contains(&pos.pt) {
                        commands.entity(entity).despawn_recursive();
                    }
                });
            }
        }

        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);
        commands.insert_resource(TurnState::Ticking);
    }
}
