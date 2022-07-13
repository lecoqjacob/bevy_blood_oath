use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    Start,
    WaitingForInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    Render,

    // The first stage (player input) is the standard Update
    PlayerCombat,
    MovePlayer,
    PlayerFov,
    GenerateMonsterMoves,
    MonsterCombat,
    MoveMonsters,
    MonsterFov,
}
