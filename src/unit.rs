use rand::Rng;
use rand::rngs::ThreadRng;

/// Different types of Units
#[derive(Clone, Copy, PartialEq)]
pub enum UnitType {
    Rock,
    Paper,
    Scissor,
    None,
}

/// Struct used as a coordinate.
#[derive(Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

/// Struct representing a Unit/Character
#[derive(Clone, Copy)]
pub struct Unit {
    coordinates: Point,
    pub unit_type: UnitType,
    pub prey: UnitType,
    speed: f32,
    size: f32,
}

impl Unit {
    /// Returns new Units
    pub fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        
        // Randomize coordinates
        let x: f32 = rng.gen_range(0.0..1260.0);
        let y: f32 = rng.gen_range(0.0..700.0);

        // Randomize unit type
        let unit_type: UnitType = match rng.gen_range(0..3) {
            0 => UnitType::Rock,
            1 => UnitType::Paper,
            2 => UnitType::Scissor,
            _ => UnitType::None,
        };

        // Set prey based on self's unit type
        let prey: UnitType = match unit_type {
            UnitType::Rock => UnitType::Scissor,
            UnitType::Paper => UnitType::Rock,
            UnitType::Scissor => UnitType::Paper,
            UnitType::None => UnitType::None,
        };

        Self { coordinates: Point { x: x, y: y }, unit_type: unit_type, prey: prey, speed: 1.0, size: 20.0 }
    }

    /// Moves the unit towards another Unit.
    pub fn move_to(&mut self, other: &Unit, dt: u128) {
        let dx: f32 = other.coordinates.x - self.coordinates.x;
        let dy: f32 = other.coordinates.y - self.coordinates.y;
        let length: f32 = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        self.coordinates.x += dx / length * self.speed * (dt / 1_000_000_000) as f32;
        self.coordinates.y += dy / length * self.speed * (dt / 1_000_000_000) as f32;
    }

    /// Attempts to attack another Unit.
    pub fn try_attack(&self, units: &mut Vec<Unit>) {
        for unit in units {
            if self.overlaps_with(unit) {
                if unit.unit_type == self.prey {
                    unit.switch_to(self.unit_type);
                }
            }
        }
    }

    /// Converts a unit to a different type
    pub fn switch_to(&mut self, unit_type: UnitType) {
        match unit_type {
            UnitType::Rock => {
                self.unit_type = unit_type;
                self.prey = UnitType::Scissor;
            }
            UnitType::Paper => {
                self.unit_type = unit_type;
                self.prey = UnitType::Rock;
            }
            UnitType::Scissor => {
                self.unit_type = unit_type;
                self.prey = UnitType::Paper;
            }
            UnitType::None => {
                self.unit_type = unit_type;
                self.prey = UnitType::None;
            }
        }
    }

    /// Calculate the Euclidian distance to other Unit.
    pub fn distance_to(&self, other: &Unit) -> f32 {
        ((self.coordinates.x - other.coordinates.x).powf(2.0) + (self.coordinates.y - other.coordinates.y).powf(2.0)).sqrt()
    }

    /// Checks if self overlaps with another Unit.
    fn overlaps_with(&self, other: &Unit) -> bool {
        return self.coordinates.x < other.coordinates.x + self.size
            && self.coordinates.x + self.size > other.coordinates.x 
            && self.coordinates.y > other.coordinates.y + self.size
            && self.coordinates.y + self.size < other.coordinates.y
    }
}