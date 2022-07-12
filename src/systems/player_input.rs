use crate::prelude::*;

pub fn player_input_system(
    mut commands: Commands,
    map: Res<Map>,
    enemies_query: Query<(Entity, &PointC), (With<Enemy>, With<CombatStats>, Without<Player>)>,
    mut player_query: Query<
        (Entity, &mut PointC, &mut FieldOfView),
        (With<Player>, Without<Enemy>),
    >,
    mut attack_events: EventWriter<WantsToAttack>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
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

        let (player_entity, mut pp, mut player_fov) = player_query.single_mut();

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            let destination = pp.0 + delta;

            for (entity, pos) in enemies_query.iter() {
                if pos.0 == destination {
                    hit_something = true;

                    attack_events.send(WantsToAttack {
                        victim: entity,
                        attacker: player_entity,
                    });
                }
            }

            if !hit_something && map.can_enter_tile(destination) {
                pp.0 = destination;
                player_fov.is_dirty = true;
            }
        }

        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);

        commands.insert_resource(NextState(TurnState::PlayerTurn));
    }
}
