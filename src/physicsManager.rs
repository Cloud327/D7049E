use std::{sync::RwLock, borrow::Borrow};

use rapier3d::{prelude::*, crossbeam::{channel::{Sender, Receiver}, self}, data::Index};


pub struct PhysicsManager{
    gravity: rapier3d::na::Matrix<f32, rapier3d::na::Const<3>, rapier3d::na::Const<1>, rapier3d::na::ArrayStorage<f32, 3, 1>>,
    integrationParameters: IntegrationParameters,
    physicsPipeline: PhysicsPipeline,
    islandManager: IslandManager,
    broadPhase: BroadPhase,
    narrowPhase: NarrowPhase,
    impulseJointSet: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccdSolver: CCDSolver,
    physicsHooks: (),
    eventHandler: ChannelEventCollector,
    rigidBodySet: RigidBodySet,
    colliderSet: ColliderSet,
    //collisionSend: Sender<CollisionEvent>,
    collisionRecieve: Receiver<CollisionEvent>
    
}


impl PhysicsManager{
    pub fn new(gravity: (f32, f32, f32)) -> Self {
        let (sender, reciever) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(sender);
        Self {
            /* Create other structures necessary for the simulation. */
            gravity: vector![gravity.0, gravity.1, gravity.2],
            //gravity: vector![0.0, -1.62, 0.0],
            integrationParameters: IntegrationParameters::default(),
            physicsPipeline: PhysicsPipeline::new(),
            islandManager: IslandManager::new(),
            broadPhase: BroadPhase::new(),
            narrowPhase: NarrowPhase::new(),
            impulseJointSet: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccdSolver: CCDSolver::new(),
            physicsHooks: (),
            
            rigidBodySet: RigidBodySet::new(),
            colliderSet: ColliderSet::new(),

            // Initialize the event collector.
            //collisionSend: sender,
            collisionRecieve: reciever,
            eventHandler: event_handler,
            //eventHandler: ChannelEventCollector::new(),

            
        }
    }

    pub fn step(&mut self){
        self.physicsPipeline.step(
            &self.gravity,
            &self.integrationParameters,
            &mut self.islandManager,
            &mut self.broadPhase,
            &mut self.narrowPhase,
            &mut self.rigidBodySet,
            &mut self.colliderSet,
            &mut self.impulseJointSet,
            &mut self.multibody_joint_set,
            &mut self.ccdSolver,
            &self.physicsHooks,
            &self.eventHandler,
        );
    }

    pub fn addRigidBody(&mut self, rigidBody: RigidBody) -> RigidBodyHandle{
        return self.rigidBodySet.insert(rigidBody);
    }

    pub fn addCollider(&mut self, collider: Collider) -> ColliderHandle{
        return self.colliderSet.insert(collider);
    }

    pub fn addColliderWithParent(&mut self, collider: Collider, parent: RigidBodyHandle) -> ColliderHandle{
        return self.colliderSet.insert_with_parent(collider, parent, &mut self.rigidBodySet);
    }

    pub fn getRigidBody(&mut self, rigidBodyHandle: RigidBodyHandle) -> Option<&mut RigidBody>{
        return self.rigidBodySet.get_mut(rigidBodyHandle);
    }

    pub fn getCollider(&self, colliderHandle: ColliderHandle) -> &Collider{
        return &self.colliderSet[colliderHandle];
    }

    pub fn removeCollider(&mut self, handle: ColliderHandle){
        self.colliderSet.remove(handle, &mut self.islandManager, &mut self.rigidBodySet, true);
    }


    pub fn removeRigidBodyWithCollider(&mut self, handle: Index){
        self.rigidBodySet.remove(RigidBodyHandle(handle), &mut self.islandManager, &mut self.colliderSet, &mut self.impulseJointSet, &mut self.multibody_joint_set, true);

    }


    pub fn getEvent(&mut self) -> Option<rapier3d::geometry::CollisionEvent>{
        while let Ok(collisionEvent) = self.collisionRecieve.try_recv() {
            // Handle the collision event.
            println!("Received collision event: {:?}", collisionEvent);
            return Some(collisionEvent);
        }
        return None;
    }



}