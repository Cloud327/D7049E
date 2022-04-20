#![allow(non_snake_case)]

mod assetTest;
mod ECS;

use ECS::gameObject;
use ECS::{healthComponent, moveComponent, attackComponent};


fn main() {
    println!("Hello, world!");
    //gameObject::main();
    //assetTest::test();
    healthComponent::test();
    println!("-----");
    moveComponent::test();
    println!("-----");
    attackComponent::test();
}
