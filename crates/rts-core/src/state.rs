#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnitId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    pub id: UnitId,
    pub position: Vec2,
    pub health: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GameTime {
    pub tick: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub time: GameTime,
    pub units: Vec<Unit>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            time: GameTime::default(),
            units: vec![
                Unit {
                    id: UnitId(1),
                    position: Vec2::new(-3.0, 0.0),
                    health: 100,
                },
                Unit {
                    id: UnitId(2),
                    position: Vec2::new(3.0, 2.0),
                    health: 100,
                },
            ],
        }
    }
}
