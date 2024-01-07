mod gui;
mod unit;
mod world;

use crate::world::SimWorld;

fn main() {
    let mut world: SimWorld = SimWorld::new();

    world.populate(1_000_000);
    world.run_sim();
}
