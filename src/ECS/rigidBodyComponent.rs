use nalgebra::{Matrix4, Translation3};
use rapier3d::{prelude::RigidBody, math::Real, prelude::Vector};


/*  
 *  Stores a rigid body pointer that points to a rigid body object contained inside 
 *  rigidBodySet inside the physicsManager
 */
pub struct RigidBodyComponent<'a>{
    rigidBody: &'a RigidBody 

}


impl RigidBodyComponent<'_>{
    pub fn new(body: &'static RigidBody)-> Self{ // Att det är static kanske är omegadåligt
        Self{
            rigidBody:body
        }
    }

    pub fn getTranslation(&self) -> Translation3<f32>{
        Translation3::new(self.rigidBody.translation()[0], self.rigidBody.translation()[1], self.rigidBody.translation()[2])
    }

    pub fn update(){
        
    }


}