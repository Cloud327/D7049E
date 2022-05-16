use kiss3d::window::Window;
use rapier3d::{prelude::*, crossbeam};
use rapier3d::pipeline::ActiveEvents;

use crate::physicsManager::PhysicsManager;

pub fn run() {
  let mut physicsManager = PhysicsManager::new((0.0, -9.81, 0.0));
  let mut window = Window::new("Collider test");

  /* Create the ground. */
  let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
  physicsManager.addCollider(collider);
  window.add_cube(10.0, 10.0, 10.0).set_color(0.7, 0.3, 0.9);

  /* Create the bounding ball. */
  let rigid_body = RigidBodyBuilder::dynamic()
          .translation(vector![0.0, 10.0, 0.0])
          .build();
  let collider = ColliderBuilder::ball(0.5).restitution(0.7).active_events(ActiveEvents::COLLISION_EVENTS).build();
  let ball_body_handle = physicsManager.addRigidBody(rigid_body);
  physicsManager.addColliderWithParent(collider, ball_body_handle);


  /* Run the game loop, stepping the simulation once per frame. */
  for _ in 0..200 {
    physicsManager.step();
    physicsManager.getEvent();
    window.render();

    let ball_body = physicsManager.getRigidBody(ball_body_handle);
    println!(
      "Ball altitude: {}",
      ball_body.unwrap().translation().y
    );
  }
}