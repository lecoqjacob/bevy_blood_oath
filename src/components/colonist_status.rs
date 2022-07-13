use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ColonistStatus {
    Alive,
    StartedDead,
    DiedAfterStart,
    Rescued,
    Unknown,
}
