use crate::unit::Unit;
use std::time::Instant;

/// Struct representing the world used for the simulation.
pub struct SimWorld {
    units: Vec<Unit>,
    last_update: Instant,
}

impl SimWorld {
    /// Returns a new world to simulate.
    pub fn new() -> Self {
        Self { units: Vec::new(), last_update: Instant::now() }
    }

    /// Continuously run simulation.
    pub fn run_sim(&mut self) {
        loop {
            self.simulate_one_cycle();
        }
    }

    /// Populate world for simulation.
    pub fn populate(&mut self, population: i32) {
        for _ in 0..population {
            let unit: Unit = Unit::new();
            self.units.push(unit);
        }
    }

    /// Runs one cycle of the simulation.
    pub fn simulate_one_cycle(&mut self) {
        let dt: u128 = self.get_dt();

        for i in 0..self.units.len() {
            let mut unit: Unit = self.units[i];
            let mut nearest: Unit = self.units[i]; // Set to self to avoid uninitialized error.
            let mut nearest_distance: f32 = 0.0;


            for j in 0..self.units.len() {
                if self.units[j].unit_type == self.units[i].prey {
                    let distance: f32 = unit.distance_to(&self.units[j]);

                    if distance > nearest_distance {
                        nearest = self.units[j];
                        nearest_distance = distance;
                    }
                }
            }

            unit.move_to(&nearest, dt);
            //unit.try_attack(&mut self.units);
        }

        println!("{dt} nano seconds")
    }

    /// Updates the last_update field and returns the new delta time. 
    fn get_dt(&mut self) -> u128 {
        let dt: u128 = self.last_update.elapsed().as_nanos();
        self.last_update = Instant::now();

        return dt
    }
}