use crate::prelude::*;

#[derive(Debug)]
pub enum TriggerType {
    EndGame,
}

#[derive(Component, Debug)]
pub struct TileTrigger(pub TriggerType);
