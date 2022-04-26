#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
use ECS::gameObject::{GameObject, self};
use ECS::transformComponent::{self, TransformComponent};
use ECS::healthComponent::{self, HealthComponent};
use kiss3d::nalgebra as na;
mod ECS;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use na::{Point3, UnitQuaternion, Vector3, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;

use crate::ECS::componentManager::ComponentManager;
use crate::ECS::manager::Manager;

const TRANSFORMCOMPONENT_ID: i32= 327;



fn main() {
    let mut manager = Manager{objects:Vec::new()};
    let mut componentManager = ComponentManager{healthComponents:Vec::new(), healthFreeList:Vec::new()};
    manager.createObject("RedTower".to_string());
    //let mut go = GameObject{componentIndices:Vec::new()};

    //let go = manager.objects[0].1;
    componentManager.healthComponents.push(HealthComponent{health: 69});
    let obj = manager.getObject(0);
    manager.getObject(0).createComponent("HealthComponent".to_string(), 0);
    let index = manager.getObject(0).getComponentIndex("HealthComponent".to_string()).unwrap();
    
    println!("{}", componentManager.healthComponents[index].health);
    

    //go.createHealthComponent(&mut componentManager, HealthComponent{health:65});

    //let comp = go.getComponent(componentManager, "HealthComponent".to_string()).unwrap();
    



    //println!("{}", comp.health);


    //gameObject::main();
    //assetTest::test();
    //println!("-----");
    //healthComponent::test();
    //println!("-----");
    //moveComponent::test();
    //println!("-----");
    //attackComponent::test();
}


