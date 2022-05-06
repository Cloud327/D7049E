use nalgebra::Matrix4;
use rapier3d::prelude::RigidBody;


// Stores a rigid body pointer that points to a rigid body object contained inside 
// rigidBodySet inside the physicsManager
pub struct RigidBodyComponent{
    //rigidBody: &'a RigidBody 

}


impl RigidBodyComponent{
    pub fn new(body: &'_ RigidBody)-> Self{
        Self{
            //rigidBody:body
        }
    }

    pub fn update(){
        
    }


}