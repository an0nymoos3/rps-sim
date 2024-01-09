mod app;
mod unit;
mod world;

use crate::world::SimWorld;
use crate::app::run_app;

fn main() {
    let mut world: SimWorld = SimWorld::new();

    world.populate(100);
    //world.run_sim();

    run_app(world);
}
