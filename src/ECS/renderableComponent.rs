use std::{rc::Rc, cell::RefCell};

use kiss3d::resource::Mesh;


// Stores a mesh of the object
// A mesh can be obtained from meshHandler
pub struct RenderableComponent{
    // Maybe change to Vec of meshes
    mesh: Rc<RefCell<Mesh>>,
}

impl RenderableComponent{
    pub fn new(mesh: Rc<RefCell<Mesh>>) -> Self {
        Self{
            mesh: mesh
        }
    }

    pub fn getMesh(&self) -> &Rc<RefCell<Mesh>>{
        return &self.mesh
    }
}