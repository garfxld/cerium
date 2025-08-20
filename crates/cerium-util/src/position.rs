#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
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
}
