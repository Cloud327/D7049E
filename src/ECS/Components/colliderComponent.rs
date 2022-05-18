use rapier3d::prelude::{ColliderHandle};

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

}