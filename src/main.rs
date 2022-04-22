#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::{gameObject, healthComponent, moveComponent, attackComponent};
use std::collections::HashMap;
use std::boxed::Box;

use crate::ECS::gameObject::{BaseComponent, GameObject};
use crate::ECS::healthComponent::HealthComponent;
use std::any::Any;
use downcast_rs::Downcast;


fn main() {
    println!("Hello, world!");

    let healthComponent = HealthComponent{health:65};


    let go = GameObject{components:vec![("HealthComponent".to_string(), Box::new(healthComponent))]};


    let resObj = go.getComponent("HealthComponent".to_string());
    let resObj = resObj.unwrap();

    let resObj = resObj.as_any().downcast_ref::<HealthComponent>();
    
    println!("{}", resObj.unwrap().health);

    //gameObject::main();
    //assetTest::test();
    healthComponent::test();
    println!("-----");
    moveComponent::test();
    println!("-----");
    attackComponent::test();
}
