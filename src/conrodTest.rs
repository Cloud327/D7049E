extern crate conrod;
extern crate find_folder;
use conrod::{widget, Positionable, Colorable, Widget, widget_ids};
use conrod::backend::glium::glium::{self, Surface};
use kiss3d::renderer::Renderer;
use conrod::{color, Labelable, Sizeable};


pub fn test() {

    let assets = find_folder::Search::KidsThenParents(3, 5)
    .for_folder("resources")
    .unwrap();
    let font_path = assets.join("fonts/NotoSans-Regular.ttf");
    


    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    ui.fonts.insert_from_file(font_path).unwrap();


    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
                    .with_title("Hello Conrod")
                    .with_dimensions(WIDTH as u32, HEIGHT as u32);
    let context = glium::glutin::ContextBuilder::new()
                    .with_vsync(true)
                    .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    //widget_ids!(struct Ids { text });
    let mut ids = Ids::new(ui.widget_id_generator());
    //let ids = Ids::new(ui.widget_id_generator());

    let mut event_loop = EventLoop::new();

    

    'main: loop {

        for event in event_loop.next(&mut events_loop){
            if let Some(event) = conrod::backend::winit::convert_event(
                event.clone(),
                &display
            ) {
                ui.handle_event(event);
            }
            match event {
                glium::glutin::Event::WindowEvent { event, ..} => match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
            
        }


        {
            let ui = &mut ui.set_widgets();

            widget::Canvas::new()
            .flow_down(&[
                (
                    ids.body,
                    widget::Canvas::new().length(HEIGHT as f64).flow_right(&[
                        (
                            ids.left_column,
                            widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0),
                        ),
                        (
                            ids.right_column,
                            widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0),
                        ),
                    ]),
                ),
            ])
            .set(ids.master, ui);

            widget::Text::new("Top Left")
                .color(color::LIGHT_ORANGE.complement())
                .top_left_of(ids.left_column)
                .set(ids.top_left, ui);

            widget::Text::new("Bottom Right")
                .color(color::DARK_ORANGE.complement())
                .bottom_right_of(ids.right_column)
                .set(ids.bottom_right, ui);

            let button = widget::Button::new().color(color::RED).w_h(50.0, 50.0);
            for _click in button.clone().middle_of(ids.left_column).set(ids.bing, ui) {
                println!("Bing!");
            }
            for _click in button.middle_of(ids.right_column).set(ids.bong, ui) {
                println!("Bong!");
            }
        }
        //     // "Hello World!" in the middle of the screen.
        //     widget::Text::new("Hello World!")
        //         .middle_of(ui.window)
        //         .color(conrod::color::WHITE)
        //         .font_size(32)
        //         .set(ids.text, ui);
        // }


        

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.6, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

    }




}



// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        body,
        left_column,
        middle_column,
        right_column,

        title,
        subtitle,
        top_left,
        bottom_right,
        bing,
        bong,

        text,
    }
}


pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop { last_update: std::time::Instant::now(),
                    ui_needs_update: true,
                  }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) ->
                Vec<glium::glutin::Event> {

        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the UI does not need updating, wait
        // for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| { events.push(event);
                                    glium::glutin::ControlFlow::Break });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether
    /// or not there are any pending events.
    ///
    /// This is primarily used on the occasion that some part of the UI is
    /// still animating and requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}