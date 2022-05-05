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

    let mut towerType = "Red";
    let mut enemyType = "Black";


    while window.render() {

        let mut mousePos = na::Point2::new(0.0f32, 0.0f32);

        for event in window.events().iter(){
            match event.value {
                WindowEvent::MouseButton(button, Action::Press, modif) => {
                    println!("mouse press event on {:?} with {:?}", button, modif);
                    
                }

                // Spawn object at mouse position
                WindowEvent::MouseButton(button, Action::Release, modif) => {
                    println!("mouse release event on {:?} with {:?}", button, modif);
                    
                }
                WindowEvent::Key(key, action, modif) => {
                    println!("key event {:?} on {:?} with {:?}", key, action, modif);

                    if matches!(Key::Numpad1, key){
                        // Switch to tower type 1
                        towerType = "Red"
                    }
                    else if matches!(Key::Numpad2, key){
                        // Switch to tower type 2
                        towerType = "White"
                    }

                    else if matches!(Key::Key1, key){
                        // Switch to enemy type 1
                        enemyType = "Blue"
                    }
                    else if matches!(Key::Key2, key){
                        // Switch to enemy type 2
                        enemyType = "Black"
                    }

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