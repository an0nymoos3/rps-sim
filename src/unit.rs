/// Different types of Units
pub enum UnitType {
    Rock,
    Paper,
    Scissor,
}

/// Struct used as a coordinate.
struct Point {
    x: i32,
    y: i32,
}

/// Struct representing a Unit/Character
pub struct Unit {
    coordinate: Point,
    unit_type: UnitType,
    prey: UnitType,
}

impl Unit {
    /// Returns new Units
    pub fn new() -> Self {
        Self { coordinate: Point {x: 0, y: 0}, unit_type: UnitType::Rock, prey: UnitType::Scissor }
    }
}