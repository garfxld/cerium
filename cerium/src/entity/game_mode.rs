#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub fn from_id(id: i32) -> Option<GameMode> {
        match id {
            0 => Some(GameMode::Survival),
            1 => Some(GameMode::Creative),
            2 => Some(GameMode::Adventure),
            3 => Some(GameMode::Spectator),
            _ => None,
        }
    }
}
