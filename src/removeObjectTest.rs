use kiss3d::camera;
use kiss3d::event::{Action, Key};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use ::nalgebra::{Translation3, Point3};
use rapier3d::data::Index;
use rapier3d::{prelude::*, crossbeam};
use rapier3d::pipeline::ActiveEvents;

use crate::physicsManager::{PhysicsManager, self};

static mut REMOVED1: bool = false;
static mut REMOVED2: bool = false;


fn createBall(r: f32, pos: (f32, f32, f32), physicsManager: &mut PhysicsManager, window: &mut Window) -> (RigidBodyHandle, SceneNode){
  /* Create the bounding ball. */
  let rigid_body = RigidBodyBuilder::dynamic()
  .translation(vector![pos.0, pos.1, pos.2])
  .build();
  let collider = ColliderBuilder::ball(r).restitution(0.7).active_events(ActiveEvents::COLLISION_EVENTS).build();
  let ball_body_handle = physicsManager.addRigidBody(rigid_body);
  physicsManager.addColliderWithParent(collider, ball_body_handle);
  let mut sphere = window.add_sphere(r);

  return (ball_body_handle, sphere);

}

pub fn updateBalls(ball_body_handle1: RigidBodyHandle, ball_body_handle2: RigidBodyHandle, physicsManager: &mut PhysicsManager, sphere1: &mut SceneNode, sphere2: &mut SceneNode, window: &mut Window){
    unsafe{
      if !REMOVED1{
        let ball_body = physicsManager.getRigidBody(ball_body_handle1);
        let t = ball_body.unwrap().translation();
        sphere1.set_local_translation(Translation3::new(t[0], t[1], t[2]));
      }
    }
    
    unsafe{
      if !REMOVED2{
        let ball_body = physicsManager.getRigidBody(ball_body_handle2);
        let t = ball_body.unwrap().translation();
        sphere2.set_local_translation(Translation3::new(t[0], t[1], t[2]));
      }
    }
    
    
}

pub fn run() {
  let mut physicsManager = PhysicsManager::new((0.0, -0.3, 0.0));
  let mut window = Window::new("Collider test");

  let eye = Point3::new(10.0, 10.0, 10.0);
  let at = Point3::new(0.0, 0.0, 0.0);
  let mut camera = camera::FirstPerson::new(eye, at);

  /* Create the ground. */
  let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();

  let groundColliderHandle = physicsManager.addCollider(collider);
  window.add_cube(100.0, 0.1, 100.0).set_color(0.7, 0.3, 0.9);

  let (ball_body_handle1, mut sphere1) = createBall(0.5, (0.0, 10.0, 0.0), &mut physicsManager, &mut window);
  let (ball_body_handle2, mut sphere2) = createBall(0.5, (0.0, 100.0, 0.0), &mut physicsManager, &mut window);

  /* Run the game loop, stepping the simulation once per frame. */
  loop{
    physicsManager.step();
    let collisionEvent = physicsManager.getEvent();
    window.render_with_camera(&mut camera);
    match collisionEvent{
      Some(collisionEvent) =>  {
        if (collisionEvent.collider1() == groundColliderHandle){

          if 1 == collisionEvent.collider2().0.into_raw_parts().0{
            window.remove_node(&mut sphere1);
            unsafe{
              REMOVED1 = true;
            }
            
           } else if 2 == collisionEvent.collider2().0.into_raw_parts().0{
             window.remove_node(&mut sphere2);
             unsafe{
               REMOVED2 = true;
             }
            
          }
          println!("Index to remove: {}", collisionEvent.collider2().0.into_raw_parts().0);
          physicsManager.removeRigidBodyWithCollider(collisionEvent.collider1().0);
        }
        
      }
      None => updateBalls(ball_body_handle1, ball_body_handle2, &mut physicsManager, &mut sphere1, &mut sphere2, &mut window),
    }

   
    

    
    //println!(
    //  "Ball altitude: {}",
      //ball_body.unwrap().translation().y
    //);

    let escape = window.get_key(Key::Escape);
            if matches!(escape, Action::Press){
                break;
            }   

  }
  
}