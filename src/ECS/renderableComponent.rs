use std::{rc::Rc, cell::RefCell};

use kiss3d::resource::Mesh;



pub struct RenderableComponent{
    mesh: Rc<RefCell<Mesh>>
}

impl RenderableComponent{
    pub fn new(mesh: Rc<RefCell<Mesh>>) -> Self {
        Self{
            mesh: mesh
        }
    }

    pub fn getMesh(&self){
        
    }
}