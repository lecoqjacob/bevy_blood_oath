use crate::prelude::*;

pub fn end_turn(mut commands: Commands, turn_state: Res<TurnState>) {
    match *turn_state {
        TurnState::Start => commands.insert_resource(TurnState::WaitingForInput),
        TurnState::PlayerTurn => commands.insert_resource(TurnState::MonsterTurn),
        TurnState::MonsterTurn => commands.insert_resource(TurnState::WaitingForInput),
        TurnState::WaitingForInput => {}
        _ => {}
    }
}
