use rapier3d::prelude::*;


pub struct PhysicsManager{
    gravity: rapier3d::na::Matrix<f32, rapier3d::na::Const<3>, rapier3d::na::Const<1>, rapier3d::na::ArrayStorage<f32, 3, 1>>,
    integrationParameters: IntegrationParameters,
    physicsPipeline: PhysicsPipeline,
    islandManager: IslandManager,
    broadPhase: BroadPhase,
    narrowPhase: NarrowPhase,
    impulseJointSet: JointSet,
    //let mut multibody_joint_set = JointSet::new();
    ccdSolver: CCDSolver,
    physicsHooks: (),
    eventHandler: (),
    rigidBodySet: RigidBodySet,
    colliderSet: ColliderSet
}


impl PhysicsManager{
    pub fn new() -> Self {
        Self {
            /* Create other structures necessary for the simulation. */
            //let gravity = vector![0.0, -9.81, 0.0];
            gravity: vector![0.0, -1.62, 0.0],
            integrationParameters: IntegrationParameters::default(),
            physicsPipeline: PhysicsPipeline::new(),
            islandManager: IslandManager::new(),
            broadPhase: BroadPhase::new(),
            narrowPhase: NarrowPhase::new(),
            impulseJointSet: JointSet::new(),
            //let mut multibody_joint_set = JointSet::new();
            ccdSolver: CCDSolver::new(),
            physicsHooks: (),
            eventHandler: (),
            rigidBodySet: RigidBodySet::new(),
            colliderSet: ColliderSet::new(),
            
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
            //&mut multibody_joint_set,
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






}