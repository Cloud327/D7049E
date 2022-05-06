use kiss3d::resource::{MeshManager};
use std::{path::Path, cell::RefMut};
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
    pub fn getNodes(&mut self, objectType: TypeEnum) -> Option<(MeshManager, Vec<String>)>{
        for mut tup in self.nodeList{
            if matches!(tup.0, objectType){
                return Some(tup.1);
            }
        }
        return None;
    } 



}
