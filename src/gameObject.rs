extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::MeshManager;
use kiss3d::resource::Mesh;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use na::{Point3, Vector3};


pub fn main(){

    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);

    let path = Path::new("./examples/test.obj");
    let path2 = Path::new("./examples/test.mtl");

    let mut meshManager = MeshManager::new();
    let mut m = MeshManager::load_obj( path, path2, "ObjectName");

    match m {
        Ok(v) => println!("working with version:"),
        Err(e) => println!("error parsing header: {:?}", e),
    }

    window.add_mesh(meshManager.get("ObjectName").unwrap(), Vector3::new(1.0, 1.0, 1.0));

    while window.render() {
        
    }
}
