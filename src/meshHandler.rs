use kiss3d::resource::{Mesh, MeshManager};
use std::{rc::Rc, cell::RefCell, path::Path};
use super::meshEnum::MeshEnum;


pub struct MeshHandler{
    meshList: Vec<(MeshEnum, Vec<Rc<RefCell<Mesh>>>)>,
}

impl MeshHandler{
    pub fn new() -> Self {
        Self{
            meshList: Vec::new(),
        }
    }

    // Create mesh from path and add to meshList
    pub fn addMesh(&mut self, meshType: MeshEnum, path1: &Path, path2: &Path){
        let mut tempList: Vec<Rc<RefCell<Mesh>>> = Vec::new();
        MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            tempList.push(mesh);
            //meshManager.add(mesh, &name[..]);
        });
        self.meshList.push((meshType, tempList));
    }

    // iterate through meshList and return the mesh with matching meshEnum type
    pub fn getMesh(&self, meshType: MeshEnum) -> Option<Vec<Rc<RefCell<Mesh>>>>{
        for (meshEnum, meshVec) in &self.meshList{
            if matches!(meshEnum, meshType){
                return Some(meshVec.to_vec())
            }
        }
        return None

    } 
}
