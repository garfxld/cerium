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
            x: x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_y(&self, y: f64) -> Self {
        Self {
            x: self.x,
            y: y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }

    pub const fn with_z(&self, z: f64) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: z,
            yaw: self.yaw,
            pitch: self.pitch,
        }
    }
}

/// A utility trait for converting a type into a [Position].
pub trait IntoPosition {
    /// Converts `self` into a [`Position`].
    fn into_position(self) -> Position;
}

impl IntoPosition for Position {
    fn into_position(self) -> Position {
        self
    }
}

impl<A> IntoPosition for (A, A, A)
where
    A: Into<f64>,
{
    fn into_position(self) -> Position {
        Position {
            x: self.0.into(),
            y: self.1.into(),
            z: self.2.into(),
            ..Position::ZERO
        }
    }
}

impl<A, B> IntoPosition for (A, A, A, B, B)
where
    A: Into<f64>,
    B: Into<f32>,
{
    fn into_position(self) -> Position {
        Position {
            x: self.0.into(),
            y: self.1.into(),
            z: self.2.into(),
            yaw: self.3.into(),
            pitch: self.4.into(),
        }
    }
}
