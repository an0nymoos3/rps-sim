use crate::unit::Unit;

/// Struct representing the world used for the simulation.
pub struct SimWorld {
    units: Vec<Unit>,
}

impl SimWorld {
    /// Returns a new world to simulate.
    pub fn new() -> Self {
        Self { units: Vec::new() }
    }

    /// Runs one cycle of the simulation.
    pub fn simulate_one_cycle(&self) {
        println!("Cycle run!");
    }
}