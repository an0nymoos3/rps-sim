mod gui;
mod unit;
mod world;

use crate::world::SimWorld;

fn main() {
    let world: SimWorld = SimWorld::new();

    // Testing loop
    for _ in 0..10 {
        world.simulate_one_cycle();
    }
}
