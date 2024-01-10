use fltk::app;
use fltk::app::App;
use fltk::frame::Frame;
use fltk::image::PngImage;
use fltk::window::{Window, DoubleWindow};
use fltk::prelude::{WidgetBase, ImageExt};
use fltk::prelude::WidgetExt;
use fltk::prelude::GroupExt;
use crate::world::SimWorld;

/// Starts and run GUI / simulation.
pub fn run_app(mut world: SimWorld) {
    // Create the application
    let app: App = app::App::default();

    // Create a window
    let mut wind: DoubleWindow = Window::new(0, 0, world.width as i32, world.height as i32, "RPS Sim");

    // Create a frame to display text
    let mut frame: Frame = Frame::new(1180, 0, 100, 50, "FPS Counter");
    frame.set_label_size(24);

    let mut test_frame: Frame = Frame::new(200, 200, 20, 20, "");
    let mut test_img = PngImage::load("resources/warning.png").expect("Failed to load warning image!");
    test_img.scale(20, 20, false, false);
    test_frame.set_image(Some(test_img));

    // This can be thought of as "the game loop" any logic that's not related to
    // how the application behaves goes in here.
    app::add_idle3(move |_|{
        let fps: u128 = world.simulate_one_cycle();

        frame.set_label(&format!("{fps} fps"));
        frame.redraw();
    
        //for unit in &mut world.units {
        //    unit.asset.draw();
        //}

        // sleeps are necessary when calling redraw in the event loop
        app::sleep(0.001);
    });
    

    // Make the window visible
    wind.end();
    wind.show();

    // Run the application
    app.run().expect("app.run() crashed!");
}