use std::{rc::Rc, cell::RefCell};
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

// Stores a mesh of the object
// A mesh can be obtained from meshHandler
pub struct RenderableComponent{
    // Maybe change to Vec of meshes
    sceneNode: Vec<SceneNode>,
}

impl RenderableComponent{
    pub fn new(sceneNode: Vec<SceneNode>) -> Self {
        Self{
            sceneNode: Vec::new(),
        }
    }

    pub fn update(&mut self){

    }

    pub fn getSceneNodes(&self) -> &Vec<SceneNode>{
        return &self.sceneNode;
    }
}