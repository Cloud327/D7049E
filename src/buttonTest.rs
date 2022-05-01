extern crate kiss3d;
extern crate nalgebra as na;


use kiss3d::window::{Canvas, CanvasSetup, NumSamples};
use kiss3d::{window::Window, event::Action};
use kiss3d::light::Light;
use kiss3d::event::{Key, MouseButton, WindowEvent};
use std::{path::Path, ops::Not};
use na::{Vector3};

pub fn test(){

    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);

    let mut rec = window.add_rectangle(60.0, 40.0);
    rec.set_color(0.0, 1.0, 0.0);


    while window.render() {
        
        let pointer = window.cursor_pos();
        let space = window.get_key(Key::Space);

        let draw_colour = na::Point3::new(0.5, 1.0, 0.5);
        let mut last_pos = na::Point2::new(0.0f32, 0.0f32);

        for event in window.events().iter(){
            match event.value {
                WindowEvent::MouseButton(button, Action::Press, modif) => {
                    println!("mouse press event on {:?} with {:?}", button, modif);
                    
                }
                WindowEvent::MouseButton(button, Action::Release, modif) => {
                    println!("mouse release event on {:?} with {:?}", button, modif);
                    
                }
                WindowEvent::Key(key, action, modif) => {
                    println!("key event {:?} on {:?} with {:?}", key, action, modif);
                }
                WindowEvent::CursorPos(x, y, _modif) => {
                    //last_pos = na::Point2::new(x as f32, y as f32);
                    println!("Last mouse position - x:{}, y:{}", x, y)
                }
                _ => {}
            }
        }

        
        // if matches!(pointer, None).not(){
        //     let pointer = pointer.unwrap();
        //     println!("x: {} - y: {}",pointer.0, pointer.1);
        // }
        // if matches!(space, Action::Press) & pressed.not(){
        //     pressed = true;
        //     println!("MELLANSLAG pressed!!---------------------------------------------------")
        // } else if matches!(space, Action::Release) & pressed {
        //     pressed = false;
        //     println!("MELLANSLAG released:(--------------------------------------------------")
        // }
        
    }
}