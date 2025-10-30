use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}

impl Position {
    pub const ZERO: Position = Position::new(0., 0., 0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        let yaw = Self::fix_yaw(yaw);
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub const fn add(&self, x: f64, y: f64, z: f64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn add_all(&self, value: f64) -> Self {
        Self {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_x(&self, x: f64) -> Self {
        Self {
            x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_y(&self, y: f64) -> Self {
        Self {
            x: self.x,
            y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_z(&self, z: f64) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_yaw(&self, yaw: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_pitch(&self, pitch: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch,
        }
    }

    const fn fix_yaw(yaw: f32) -> f32 {
        let mut yaw = yaw % 360.0;
        if yaw < -180.0 {
            yaw += 360.0;
        } else if yaw > 180.0 {
            yaw -= 360.0;
        }
        yaw
    }
}

impl<A> From<(A, A, A)> for Position
where
    A: Into<f64>,
{
    fn from(value: (A, A, A)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
            ..Position::ZERO
        }
    }
}

impl<A, B> From<(A, A, A, B, B)> for Position
where
    A: Into<f64>,
    B: Into<f32>,
{
    fn from(value: (A, A, A, B, B)) -> Self {
        Position {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into(),
            yaw: value.3.into(),
            pitch: value.4.into(),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone)]
    pub struct TeleportFlags: i32 {
        const NONE       = 0x0000;

        const X          = 0x0001;
        const Y          = 0x0002;
        const Z          = 0x0004;
        const YAW        = 0x0008;
        const PITCH      = 0x0010;

        const VELOCITY_X = 0x0020;
        const VELOCITY_Y = 0x0040;
        const VELOCITY_Z = 0x0080;

        const ROTATE     = 0x0100;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockPosition {
    x: i64,
    y: i64,
    z: i64,
}

impl BlockPosition {
    pub const ZERO: BlockPosition = BlockPosition::new(0, 0, 0);

    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn y(&self) -> i64 {
        self.y
    }

    pub fn z(&self) -> i64 {
        self.z
    }

    pub const fn add(&self, x: i64, y: i64, z: i64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
        }
    }
}

impl Into<BlockPosition> for Position {
    fn into(self) -> BlockPosition {
        BlockPosition::new(
            (self.x() / 16.0) as i64,
            (self.y() / 16.0) as i64,
            (self.z() / 16.0) as i64,
        )
    }
}
