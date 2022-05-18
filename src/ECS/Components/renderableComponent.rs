use std::sync::RwLock;
use std::{rc::Rc, cell::RefCell};
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

// Stores a mesh of the object
// A mesh can be obtained from meshHandler
pub struct RenderableComponent{
    // Maybe change to Vec of meshes
    sceneNode: SceneNode,
}

impl RenderableComponent{
    pub fn new(sceneNode: SceneNode) -> Self {
        Self{
            sceneNode: sceneNode,
        }

    }

    pub fn update(&mut self){

    }

    pub fn getSceneNode(&mut self) -> &mut SceneNode{
        return &mut self.sceneNode;
    }
}