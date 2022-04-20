#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::{gameObject, healthComponent, moveComponent, attackComponent};
use std::collections::HashMap;
use std::boxed::Box;
use std::convert::From;

use crate::ECS::gameObject::BaseComponent;

fn main() {
    println!("Hello, world!");
    let hpComp = healthComponent::HealthComponent{health: 60};

    //let baseComp = hpComp.into();
    
    let a: &dyn BaseComponent = &hpComp;

    let b = Box::new(a);

    let mut map = HashMap::new();
    map.insert("HealthComponent".to_string(), b);

    let go = gameObject::GameObject{components:map};
    

    //gameObject::main();
    //assetTest::test();
    healthComponent::test();
    println!("-----");
    moveComponent::test();
    println!("-----");
    attackComponent::test();
}
