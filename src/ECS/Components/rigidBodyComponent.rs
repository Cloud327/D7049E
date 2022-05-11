use nalgebra::{Matrix4, Translation3};
use rapier3d::{prelude::{RigidBody, RigidBodyHandle}, math::Real, prelude::Vector};


/*  
 *  Stores a rigid body pointer that points to a rigid body object contained inside 
 *  rigidBodySet inside the physicsManager
 */
pub struct RigidBodyComponent{
    rigidBodyHandle: RigidBodyHandle 

}


impl RigidBodyComponent{
    pub fn new(body: RigidBodyHandle)-> Self{ // Att det är static kanske är omegadåligt
        Self{
            rigidBodyHandle:body
        }
    }

    // pub fn getTranslation(&self) -> Translation3<f32>{
    //     Translation3::new(self.rigidBodyHandle.translation()[0], self.rigidBodyHandle.translation()[1], self.rigidBodyHandle.translation()[2])
    // }

    pub fn getRigidBodyHandle(&self) -> RigidBodyHandle{
        return self.rigidBodyHandle;
    }

    pub fn update(){
        
    }


}