use kiss3d::{resource::{MeshManager}, window::Window};
use std::{path::Path, cell::RefMut, borrow::Borrow};
use crate::ECS::typeEnum::TypeEnum;


pub struct NodeHandler{
    nodeList: Vec<(TypeEnum, (MeshManager, Vec<String>))>,
}

impl NodeHandler{
    pub fn new() -> Self {
        Self{
            nodeList: Vec::new(),
        }
    }

    // Create mesh from path and add to meshList
    pub fn addNodes(&mut self, objectType: TypeEnum, path1: &Path, path2: &Path){
        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<String> = Vec::new();

        let objects = MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            meshManager.add(mesh, &name[..]);
            objNames.push(name[..].to_string());
        });
        self.nodeList.push((objectType, (meshManager, objNames)));
        
    }

    // Finds the right tuple to return based on object type
    pub fn getNodes(&mut self, objectType: TypeEnum) -> Option<&(MeshManager, Vec<String>)>{
        for mut tup in &self.nodeList{
            if matches!(tup.0, objectType){
                return Some(&tup.1);
            }
        }
        return None;
    } 


    pub fn getSceneNodes(&mut self, window: Window, objectType: TypeEnum){
        
        // for temp in self.nodeList{
        //     if matches!(temp.0, objectType){
        //         let towerNodes = temp.1;
        //     }
        // }
        // let mut sceneNodes: Vec<SceneNode> = Vec::new();
        
        // for name in towerNodes.1{
        //     let mesh = towerNodes.0.get(&name);
        //     let mut temp = self.window.add_mesh(mesh.unwrap(), Vector3::new(1.0, 1.0, 1.0));
        //     temp.set_local_translation(Translation3::new(x as f32, y as f32, z as f32));
        //     sceneNodes.push(temp);
        // }
    }



}