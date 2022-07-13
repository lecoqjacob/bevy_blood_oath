use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    Setup,
    Start,
    WaitingForInput,
    PlayerTurn,
    AITurn,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    GenerateAIMoves,
    PlayerStage,
    AIStage,
}
