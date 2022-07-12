use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnState {
    Setup,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    RenderMap,
    RenderEntities,
    RenderUi,
    Render,
    CameraMove,

    // The first stage (player input) is the standard Update
    PlayerCombat,
    MovePlayer,
    PlayerFov,
    GenerateMonsterMoves,
    MonsterCombat,
    MoveMonsters,
    MonsterFov,
}
