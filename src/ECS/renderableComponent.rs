use std::sync::RwLock;
use std::{rc::Rc, cell::RefCell};
use kiss3d::scene::SceneNode;
use kiss3d::resource::Mesh;

// Stores a mesh of the object
// A mesh can be obtained from meshHandler
pub struct RenderableComponent{
    // Maybe change to Vec of meshes
    sceneNode: RwLock<SceneNode>,
}

impl RenderableComponent{
    pub fn new(sceneNodes: Vec<SceneNode>) -> Self {
        // Go through the list of nodes and add them all as children to the first node
        let mut node = sceneNodes[0].clone();
        let mut first = true;
        for n in sceneNodes{
            if first == false {
                node.add_child(n);
            }else{
                first = false;
            }
        }
        Self{
            sceneNode: RwLock::new(node),
        }


    }

    pub fn update(&mut self){

    }

    pub fn getSceneNode(&self) -> &RwLock<SceneNode>{
        return &self.sceneNode;
    }
}