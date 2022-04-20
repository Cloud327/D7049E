extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::MeshManager;
use std::path::Path;
use na::{Vector3};



pub fn test(){
    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);

    let path1 = Path::new("src/resources/mushroom.obj");
    let path2 = Path::new("src/resources/mushroom.mtl");

    let mut meshManager = MeshManager::new();

    MeshManager::load_obj(&path1, &path2, "obj")
        .unwrap()
        .into_iter()
        .for_each(|(name,mesh,_)| {
            meshManager.add(mesh, &name[..]);
        });

    let mut m = window.add_mesh(meshManager.get("obj").unwrap(), Vector3::new(1.0, 1.0, 1.0));
    m.set_color(1.0, 0.0, 0.0);

    while window.render() {

    }
}