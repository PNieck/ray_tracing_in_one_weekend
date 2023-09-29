use std::ops;

use super::vec3::Vec3;


#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Position {
    pub fn zero() -> Position {
        Position { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn translation(p: Position, v: Vec3) -> Position {
        Position {
            x: p.x + v.x,
            y: p.y + v.y,
            z: p.z + v.z
        }
    }
}

impl ops::Sub for Position {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
