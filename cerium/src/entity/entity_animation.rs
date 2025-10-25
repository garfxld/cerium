#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityAnimation {
    SwingMainArm,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicalCriticalEffect,
}

impl EntityAnimation {
    pub fn id(&self) -> i32 {
        match self {
            Self::SwingMainArm => 0,
            Self::LeaveBed => 2,
            Self::SwingOffhand => 3,
            Self::CriticalEffect => 4,
            Self::MagicalCriticalEffect => 5,
        }
    }
}

impl TryFrom<i32> for EntityAnimation {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::SwingMainArm,
            2 => Self::LeaveBed,
            3 => Self::SwingOffhand,
            4 => Self::CriticalEffect,
            5 => Self::MagicalCriticalEffect,
            _ => return Err(()),
        })
    }
}
