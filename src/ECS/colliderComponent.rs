use rapier3d::prelude::Collider;

/*  
 *  TODO?
 */
pub struct ColliderComponent<'a>{
    collider: &'a Collider

}


impl ColliderComponent<'_>{
    pub fn new(collider: &'static Collider)-> Self{ // Att det är static kanske är omega dåligt
        Self{
            collider:collider
        }
    }

    pub fn update(){
        
    }

    pub fn getCollider(&self) -> &rapier3d::geometry::Collider{
        return self.collider;
    }


}

/*
pub struct RigidBodyComponent<'a>{
    rigidBody: &'a RigidBody 

}


impl RigidBodyComponent<'_>{
    pub fn new(body: &'static RigidBody)-> Self{ // Att det �r static kanske �r omega d�ligt
        Self{
            rigidBody:body
        }
    }

    pub fn getTranslation(&self) -> Translation3<f32>{
        Translation3::new(self.rigidBody.translation()[0], self.rigidBody.translation()[1], self.rigidBody.translation()[2])
    }

    pub fn update(){
        
    }


} */