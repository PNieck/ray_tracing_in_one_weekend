use super::{position::Position, vec3::Vec3};


#[derive(PartialEq, Clone, Debug)]
pub struct Ray {
    origin: Position,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Position, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}
