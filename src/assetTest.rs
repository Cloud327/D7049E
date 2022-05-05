extern crate kiss3d;
use nalgebra as na;
use kiss3d::{window::Window, resource::Mesh};
use kiss3d::light::Light;
use kiss3d::resource::MeshManager;
use std::{path::Path, rc::Rc, cell::RefCell};
use na::{Vector3};



pub fn test(){
    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);

    //let path1 = Path::new("src/resources/teapot/teapot.obj");
    //let path2 = Path::new("src/resources/teapot/default.png");

    let path1 = Path::new("src/resources/mushroom/mushroom.obj");
    let path2 = Path::new("src/resources/mushroom/mushroom.mtl");


    //window.add_obj(path1, path2, Vector3::new(1.0, 1.0, 1.0));

    
    let objects = MeshManager::load_obj(path1, path2, "obj");
    
    /*
    let objects = MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            meshManager.add(mesh, &name[..]);
        });
    
    let mut m = window.add_mesh(meshManager.get("obj").unwrap(), Vector3::new(1.0, 1.0, 1.0));
    //m.set_color(1.0, 0.0, 0.0);
     */

    for obj in objects.unwrap(){
        let mut scene = window.add_mesh(obj.1, Vector3::new(1.0, 1.0, 1.0));
        //scene.set_material_with_name(obj.2.unwrap().name.as_str())

    }
    
    while window.render() {
        
    }
}