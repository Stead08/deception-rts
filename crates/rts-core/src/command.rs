use crate::{UnitId, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    AdvanceTime { ticks: u64 },
    MoveUnit { unit_id: UnitId, destination: Vec2 },
}
