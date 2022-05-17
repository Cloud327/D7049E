use nalgebra::Matrix4;
use rapier3d::prelude::{ColliderHandle};


/*  
 *  TODO?
 */
pub struct ColliderComponent{
    colliderHandle: ColliderHandle,
}

impl ColliderComponent{
    pub fn new(collider: ColliderHandle)-> Self{ 
        Self{
            colliderHandle: collider,
        }
    }

    pub fn getColliderHandle(&self) -> ColliderHandle{
        return self.colliderHandle;
    }

    pub fn update(){
        
    }

}