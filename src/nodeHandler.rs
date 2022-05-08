use conrod::graph::Node;
use kiss3d::{resource::{MeshManager}, window::Window, scene::SceneNode};
use nalgebra::{Vector3, Translation3};
use rapier3d::{math::{Real, Point}, na::Point3};
use std::{path::Path, cell::RefMut, borrow::Borrow, sync::RwLock, ops::DerefMut};
use crate::ECS::typeEnum::TypeEnum;


pub struct NodeHandler{
    nodeList: Vec<(TypeEnum, (RwLock<MeshManager>, RwLock<Vec<RwLock<String>>>))>,
    pointList: Vec<(TypeEnum, Vec<Point<Real>>)>,
}


impl NodeHandler{
    pub fn new() -> Self {
        Self{
            nodeList: Vec::new(),
            pointList: Vec::new(),
        }
    }

    // Creates meshes from path and add to meshManager
    // 
    pub fn addNodes(&mut self, objectType: TypeEnum, path1: &Path, path2: &Path){
        let mut meshManager = MeshManager::new();
        let mut objNames: Vec<RwLock<String>> = Vec::new();
        let mut points:Vec<Point<Real>>;
        points = Vec::new();

        let objects = MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            let m = mesh.borrow_mut().coords().read().unwrap().data().clone().unwrap();
            for point in m.into_iter(){
                points.push(Point::new(point[0], point[1], point[2]));
                
            }
            meshManager.add(mesh, &name[..]);
            objNames.push(RwLock::new(name[..].to_string()));
        });
        self.pointList.push((objectType, points));
        self.nodeList.push((objectType, (RwLock::new(meshManager), RwLock::new(objNames))));
        
    }

    // Finds the right tuple to return based on object type
    pub fn getNodes(&mut self, objectType: TypeEnum) -> Option<&(RwLock<MeshManager>, RwLock<Vec<RwLock<String>>>)>{
        for mut tup in self.nodeList{
            if matches!(tup.0, objectType){
                return Some(&tup.1);
            }
        }
        return None;
    } 


    // pub fn getSceneNodes(&mut self, window: Window, objectType: TypeEnum, x: usize, y: usize, z: usize){
        
    //     let nodes = self.getNodes(objectType).unwrap();
    //     let mut sceneNodes: Vec<SceneNode> = Vec::new();
        
    //     for name in nodes.1{
    //         let mesh = nodes.0.get(&name);
    //         let mut temp = window.add_mesh(mesh.unwrap(), Vector3::new(1.0, 1.0, 1.0));
    //         temp.set_local_translation(Translation3::new(x as f32, y as f32, z as f32));
    //         sceneNodes.push(temp);
    //     }
    // }


}
