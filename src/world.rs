use crate::unit::Unit;
use std::time::Instant;

/// Struct representing the world used for the simulation.
pub struct SimWorld {
    pub units: Vec<Box<Unit>>,
    last_update: Instant,
    pub width: f32,
    pub height: f32,
}

impl SimWorld {
    /// Returns a new world to simulate.
    pub fn new() -> Self {
        Self { units: Vec::new(), last_update: Instant::now(), width: 1280.0, height: 720.0 }
    }

    /*
    For running in headless mode.

    /// Continuously run simulation.
    pub fn run_sim(&mut self) {
        loop {
            self.simulate_one_cycle();
        }
    }
    */

    /// Populate world for simulation.
    pub fn populate(&mut self, population: i32) {
        for _ in 0..population {
            let unit: Box<Unit> = Unit::new(self.width, self.height);
            self.units.push(unit);
        }
    }

    /// Runs one cycle of the simulation.
    pub fn simulate_one_cycle(&mut self) -> u128 {
        let dt: u128 = self.get_dt();
        let len: usize = self.units.len();
        let units: Vec<Box<Unit>> = self.units.clone();

        for i in 0..len {
            self.units[i].update(dt, &units);
        }

        1_000_000_000 / dt
    }

    /// Updates the last_update field and returns the new delta time. 
    fn get_dt(&mut self) -> u128 {
        let dt: u128 = self.last_update.elapsed().as_nanos();
        self.last_update = Instant::now();

        return dt
    }
}