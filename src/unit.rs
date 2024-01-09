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
#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Struct representing a Unit/Character.
#[derive(Clone)]
pub struct Unit {
    pub coordinates: Point,
    pub unit_type: UnitType,
    pub prey: UnitType,
    speed: f32,
    pub size: f32,
}

impl Unit {
    /// Heap allocates a new Unit and returns a pointer (-> Box<Unit>)
    pub fn new(max_x: f32, max_y: f32) -> Box<Self> {
        let mut rng: ThreadRng = rand::thread_rng();
        
        // Randomize coordinates
        let x: f32 = rng.gen_range(0.0..max_x - 20.0); // 20.0 = size of unit
        let y: f32 = rng.gen_range(0.0..max_y - 20.0); // 20.0 = size of unit

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

        Box::new(Self { coordinates: Point { x: x, y: y }, unit_type: unit_type, prey: prey, speed: 1.0, size: 20.0 })
    }

    /// Runs an update cycle on Unit.
    pub fn update(&mut self, dt: u128, others: &Vec<Box<Unit>>) {
        let mut nearest: Option<&Box<Unit>> = None;
        let mut nearest_distance: f32 = f32::MAX;
        
        for unit in others {
            if unit.prey == self.unit_type {
                let distance: f32 = self.distance_to(&unit);
                
                if distance < nearest_distance {
                    nearest = Some(unit);
                    nearest_distance = distance;
                }
            }
        }

        // Update position
        if let Some(other) = nearest {
            self.move_to(other, dt);
        }
        // Check whether self should change UnitType.
        self.check_switch(others);
    }

    /// Moves the unit towards another Unit.
    fn move_to(&mut self, other: &Unit, dt: u128) {
        let dx: f32 = other.coordinates.x - self.coordinates.x;
        let dy: f32 = other.coordinates.y - self.coordinates.y;
        let length: f32 = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

        self.coordinates.x += dx / length * self.speed * (dt / 1_000_000_000) as f32;
        self.coordinates.y += dy / length * self.speed * (dt / 1_000_000_000) as f32;
    }

    /// Checks if self is overlapping with a predator and switches UnitType if true
    fn check_switch(&mut self, units: &Vec<Box<Unit>>) {
        for unit in units {
            if unit.prey == self.unit_type {
                if self.overlaps_with(unit) {
                    self.switch_to(unit.prey);
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