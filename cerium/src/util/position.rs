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
