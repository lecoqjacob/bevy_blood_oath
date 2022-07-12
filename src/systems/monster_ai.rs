use crate::prelude::*;

pub fn monster_ai(
    mut map: ResMut<Map>,
    turn_state: Res<CurrentState<TurnState>>,
    player_pos_q: Query<(Entity, &PointC), (With<Player>, Without<Enemy>)>,
    mut monster_q: Query<(Entity, &mut PointC, &mut FieldOfView), (With<Enemy>, Without<Player>)>,
    mut attack_events: EventWriter<WantsToAttack>,
) {
    if turn_state.0 != TurnState::MonsterTurn {
        return;
    }

    let (player_ent, player_pos) = player_pos_q.single();

    for (entity, mut point_c, mut fov) in monster_q.iter_mut() {
        let distance = DistanceAlg::Pythagoras.distance2d(point_c.0, player_pos.0);
        if distance < 1.5 {
            attack_events.send(WantsToAttack {
                attacker: entity,
                victim: player_ent,
            });
        } else if fov.visible_tiles.contains(&player_pos.0) {
            // Path to the player
            let path = a_star_search(map.pt_idx(point_c.0), map.pt_idx(player_pos.0), &*map);

            if path.success && path.steps.len() > 1 {
                map.blocked.remove(&point_c.0);

                let destination = Point::new(
                    path.steps[1] as i32 % map.width,
                    path.steps[1] as i32 / map.width,
                );

                point_c.0 = destination;

                map.blocked.insert(destination);
                fov.is_dirty = true;
            }
        }
    }
}

// pub fn monster_ai(
//     mut commands: Commands,
//     player_pos_q: Query<&PointC, (With<Player>, Without<Enemy>)>,
//     mut movers: Query<
//         (
//             Entity,
//             &FieldOfView,
//             Option<&MovingRandomly>,
//             Option<&ChasingPlayer>,
//         ),
//         With<Enemy>,
//     >,
// ) {
//     let player_pos = player_pos_q.single();

//     for (entity, fov, randomer, chaser) in movers.iter_mut() {
//         if randomer.is_some() && fov.visible_tiles.contains(&player_pos.0) {
//             println!("Switing to chasing movement");
//             commands
//                 .entity(entity)
//                 .remove::<MovingRandomly>()
//                 .insert(ChasingPlayer {});
//         } else if chaser.is_some() && !fov.visible_tiles.contains(&player_pos.0) {
//             eprintln!("Switing to random movement");
//             commands
//                 .entity(entity)
//                 .remove::<ChasingPlayer>()
//                 .insert(MovingRandomly {});
//         }
//     }
// }
