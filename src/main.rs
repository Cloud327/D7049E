#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::{gameObject, healthComponent, moveComponent, attackComponent};
use std::collections::HashMap;
use std::boxed::Box;

use crate::ECS::gameObject::BaseComponent;


fn main() {
    println!("Hello, world!");

    //gameObject::main();
    assetTest::test();
    healthComponent::test();
    println!("-----");
    moveComponent::test();
    println!("-----");
    attackComponent::test();
}
