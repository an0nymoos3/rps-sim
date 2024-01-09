use fltk::image::PngImage;
use fltk::app;
use fltk::app::App;
use fltk::frame::Frame;
use fltk::window::{Window, DoubleWindow};
use fltk::prelude::{WidgetBase, ImageExt};
use fltk::prelude::WidgetExt;
use fltk::prelude::GroupExt;
use crate::unit::{Point, UnitType};
use crate::world::SimWorld;

/// Starts and run GUI / simulation.
pub fn run_app(mut world: SimWorld) {
    // Create the application
    let app: App = app::App::default();

    // Create a window
    let mut wind: DoubleWindow = Window::new(0, 0, world.width as i32, world.height as i32, "RPS Sim");

    // Create a frame to display text
    let mut frame: Frame = Frame::new(1180, 0,100, 50, "");
    frame.set_label_size(24);

    // Make the window visible
    wind.end();
    wind.show();

    let mut rock_img: PngImage = PngImage::load("resources/rock.png").expect("Failed to load rock image!");
    let mut paper_img: PngImage = PngImage::load("resources/paper.png").expect("Failed to load paper image!");
    let mut scissor_img: PngImage = PngImage::load("resources/scissor.png").expect("Failed to load scissor image!");

    wind.draw({ 
        for unit in &world.units {
            let pos: &Point = &unit.coordinates;
             
            let mut image: PngImage = match &unit.unit_type {
                UnitType::Rock => rock_img.clone(),
                UnitType::Paper => paper_img.clone(),
                UnitType::Scissor => scissor_img.clone(),
                UnitType::None => rock_img.clone(),
            };
        }

        move |_| {
            rock_img.draw(100, 100, 20, 20);
        }
    });

    // This can be thought of as "the game loop" any logic that's not related to
    // how the application behaves goes in here.
    app::add_idle3(move |_|{
        let fps: u128 = world.simulate_one_cycle();

        frame.set_label(&format!("{fps} fps"));
        frame.redraw();
        wind.redraw();

        // sleeps are necessary when calling redraw in the event loop
        app::sleep(0.001);
    });

    // Run the application
    app.run().expect("app.run() crashed!");
}