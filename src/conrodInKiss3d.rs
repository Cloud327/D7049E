extern crate kiss3d;
extern crate nalgebra as na;

use conrod::color::Color;
use conrod::position::Positionable;
use conrod::{widget_ids, theme};
use kiss3d::light::Light;
use kiss3d::window::Window;
use std::path::Path;

pub fn test(){
    let mut window = Window::new("Kiss3d: UI");
    window.set_background_color(1.0, 1.0, 1.0);
    let mut c = window.add_cube(0.1, 0.1, 0.1);
    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    // // Generate the widget identifiers.
    // let ids = Ids::new(window.conrod_ui_mut().widget_id_generator());
    // window.conrod_ui_mut().theme = theme();
    // window.add_texture(&Path::new("./examples/media/kitten.png"), "cat");
    // let cat_texture = window.conrod_texture_id("cat").unwrap();

    // let mut app = DemoApp::new(cat_texture);

    // Render loop.
    while window.render() {
        // let mut ui = window.conrod_ui_mut().set_widgets();
        // gui(&mut ui, &ids, &mut app)
    }
}