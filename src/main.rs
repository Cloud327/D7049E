#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::{gameObject, healthComponent, moveComponent, attackComponent};
use rust_3d::io::Header;
use crate::ECS::componentManager::{self, ComponentManager};
use crate::ECS::gameObject::{BaseComponent, GameObject};
use crate::ECS::healthComponent::HealthComponent;
use crate::ECS::manager::Manager;



fn main() {
    println!("Hello, world!");

    let mut manager = Manager{objects:Vec::new()};
    let mut componentManager = ComponentManager{healthComponents:Vec::new(), healthFreeList:Vec::new()};

    let mut go = GameObject{componentIndices:Vec::new()};
    componentManager.createHealthComponent(&mut go, HealthComponent{health:65});

    let index = go.getComponentIndex("HealthComponent".to_string());
    
    let comp = componentManager.getHealthComponent(index.unwrap());
    println!("{}", comp.health);


    //gameObject::main();
    //assetTest::test();
    println!("-----");
    healthComponent::test();
    println!("-----");
    moveComponent::test();
    println!("-----");
    attackComponent::test();
}
