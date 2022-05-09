use kiss3d::ncollide3d::narrow_phase::ContactEvent;
use rapier3d::{prelude::*, crossbeam::{self, channel::{Sender, Receiver}}};


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
    eventHandler: (),
    rigidBodySet: RigidBodySet,
    colliderSet: ColliderSet,
    collisionSend: Sender<CollisionEvent>,
    collisionRecieve: Receiver<CollisionEvent>
    
}


impl PhysicsManager{
    pub fn new() -> Self {
        // Used for 
        let (sender, reciever) = crossbeam::channel::unbounded();
        Self {
            /* Create other structures necessary for the simulation. */
            //let gravity = vector![0.0, -9.81, 0.0];
            gravity: vector![0.0, -1.62, 0.0],
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
            collisionSend: sender,
            collisionRecieve: reciever,
            eventHandler: (),
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
        self.rigidBodySet.insert(rigidBody)
    }

    pub fn addCollider(&mut self, collider: Collider){
        self.colliderSet.insert(collider);
    }

    pub fn addColliderWithParent(&mut self, collider: Collider, parent: RigidBodyHandle){
        self.colliderSet.insert_with_parent(collider, parent, &mut self.rigidBodySet);
    }

    pub fn getEvent(&mut self){
        
        //let event_handler = ChannelEventCollector::new(intersection_send, collision_send);
    }

}
