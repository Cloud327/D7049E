use kiss3d::{resource::{MeshManager}};
use rapier3d::{math::{Real, Point}};
use std::{path::Path, mem::discriminant};
use crate::ECS::typeEnum::TypeEnum;


pub struct NodeHandler{
    meshManager: Vec<MeshManager>,
    names: Vec<Vec<String>>,
    types: Vec<TypeEnum>,
}


impl NodeHandler{
    pub fn new() -> Self {
        Self{
            meshManager: Vec::new(),
            names: Vec::new(),
            types: Vec::new(),

        }
    }

    // Creates meshes from path and add to meshManager
    // 
    pub fn addNodes(&mut self, objectType: TypeEnum, path1: &Path, path2: &Path){
        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<String> = Vec::new();
        let mut points:Vec<Point<Real>> = Vec::new();

        let objects = MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            meshManager.add(mesh, &name[..]);
            objNames.push(name[..].to_string());
        });
        self.meshManager.push(meshManager);
        self.names.push(objNames);
        self.types.push(objectType);
        
    }


    // Finds the right meshManager to return based on object type
    pub fn getMeshManager(&mut self, objectType: TypeEnum) -> Option<&mut MeshManager>{
        let i = self.getIndex(objectType).unwrap();
        return self.meshManager.get_mut(i);
    } 

    
    // Finds the right name list to return based on object type
    pub fn getNames(&mut self, objectType: TypeEnum) -> Option<&Vec<String>>{
        let i = self.getIndex(objectType).unwrap();
        return self.names.get(i);
    } 


    fn getIndex(&mut self, objectType: TypeEnum) -> Option<usize>{
        let mut i = 0;
        for objType in &self.types{
            if discriminant(objType) == discriminant(&objectType){
                return Some(i);
            }
            i += 1;
        }
        return None;
    }

}
