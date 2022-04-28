use kiss3d::resource::{Mesh, MeshManager};
use std::{rc::Rc, cell::RefCell, path::Path};
use super::meshEnum::MeshEnum;


pub struct MeshHandler{
    meshList: Vec<(MeshEnum, Rc<RefCell<Mesh>>)>,
}

impl MeshHandler{
    pub fn new() -> Self {
        Self{
            meshList: Vec::new(),
        }
    }

    // Create mesh from path and add to meshList
    pub fn addMesh(&mut self, meshType: MeshEnum, path1: &Path, path2: &Path){
        MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            self.meshList.push((meshType, mesh));
            //meshManager.add(mesh, &name[..]);
        });
    }

    // iterate through meshList and return the mesh with matching meshEnum type
    pub fn getMesh(&self, meshType: MeshEnum) -> Option<Rc<RefCell<Mesh>>>{
        for (meshEnum, mesh) in self.meshList{
            if matches!(meshEnum, meshType){
                return Some(mesh)
            }
        }
        return None

    }

}