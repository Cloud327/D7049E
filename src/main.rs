#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::{gameObject, healthComponent, moveComponent, attackComponent};
use rust_3d::io::Header;
use crate::ECS::componentEnum::ComponentEnum;
use crate::ECS::componentManager::{self, ComponentManager};
use crate::ECS::gameObject::{BaseComponent, GameObject};
use crate::ECS::healthComponent::HealthComponent;
use crate::ECS::manager::Manager;



fn main() {
    println!("Hello, world!");

    let mut manager = Manager{objects:Vec::new()};
    let mut componentManager = ComponentManager{healthComponents:Vec::new(), healthFreeList:Vec::new()};

    let mut go = GameObject{componentIndices:Vec::new()};
    go.createHealthComponent(&mut componentManager, HealthComponent{health:65});

    let comp = go.getComponent(componentManager, "HealthComponent".to_string()).unwrap();
    let hpComp = ;
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
