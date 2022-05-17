use rapier3d::{prelude::*, crossbeam};
use rapier3d::pipeline::ActiveEvents;

use crate::physicsManager::PhysicsManager;

pub fn run() {
  let mut physicsManager = PhysicsManager::new();

  /* Create the ground. */
  let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
  physicsManager.addCollider(collider);

  /* Create the bounding ball. */
  let mut rigid_body = RigidBodyBuilder::dynamic()
          .translation(vector![0.0, 10.0, 0.0])
          .build();
  rigid_body.set_linvel(vector![0.0, -1.0, 0.0], true);
  let collider = ColliderBuilder::ball(0.5).restitution(0.7).active_events(ActiveEvents::COLLISION_EVENTS).build();
  let ball_body_handle = physicsManager.addRigidBody(rigid_body);
  let colliderHandle = physicsManager.addColliderWithParent(collider, ball_body_handle);
  


  /* Run the game loop, stepping the simulation once per frame. */
  for i in 0..200 {
    physicsManager.step();
    physicsManager.getEvent();
    
    
    let ball_body = physicsManager.getRigidBody(ball_body_handle).unwrap();
    println!(
      "rigidBody: {}, time: {}",
      ball_body.translation(), i
    );

    let ball_collider = physicsManager.getCollider(colliderHandle);
    println!(
      "collider: {}, time: {}",
      ball_collider.translation(), i
    );
    
  }
}