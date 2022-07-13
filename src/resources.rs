use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum TurnState {
    Start,
    WaitingForInput,
    // PlayerTurn,
    // AITurn,
    GameOverLeft,
    Ticking,
    Cleanup,
    // Modal(&'a str, &'a str),
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
