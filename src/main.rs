#![allow(non_snake_case)]


mod assetTest;
mod ECS;

use ECS::attackRateComponent::AttackRateComponent;

// use ECS::{healthComponent, moveComponent, attackComponent};
// use rust_3d::io::Header;
// use crate::ECS::componentEnum::{ComponentEnum, self};
// use crate::ECS::componentManager::{self, ComponentManager};
// use crate::ECS::gameObject::{BaseComponent, GameObject};
// use crate::ECS::healthComponent::HealthComponent;
use crate::ECS::{manager::Manager, healthComponent::HealthComponent, moveComponent::MoveComponent,attackDamageComponent::AttackDamageComponent};

// mod test;

fn main() {
    let mut manager = Manager::new();

    let redEnemy = manager.newObject();
    manager.addComponentToObject(redEnemy, HealthComponent{health:65});
    manager.addComponentToObject(redEnemy, MoveComponent::new(1));

    let blueEnemy = manager.newObject();
    manager.addComponentToObject(blueEnemy, HealthComponent{health:0});
    manager.addComponentToObject(blueEnemy, MoveComponent::new(2));

    let stationaryEnemy = manager.newObject();
    manager.addComponentToObject(stationaryEnemy, HealthComponent{health:0});

    let redTower = manager.newObject();
    manager.addComponentToObject(redTower, AttackDamageComponent::new(15));

    let blueTower = manager.newObject();
    manager.addComponentToObject(blueTower, AttackRateComponent::new(3));



    // example of how to iterate through all objects who have both a health and movecomponent
    let mut healths = manager.borrowComponentVecMut::<HealthComponent>().unwrap();
    let mut moves = manager.borrowComponentVecMut::<MoveComponent>().unwrap();
    let zip = healths.iter_mut().zip(moves.iter_mut());
    let iter = zip.filter_map(|(health, movement)| Some((health.as_mut()?, movement.as_mut()?)));

    for (health, movement) in iter {
        if health.getHealth() <= 0 {
            println!("someone dead has a speed of {}", movement.getSpeed())
        } else {
            println!("someone who has a speed of {} is alive!", movement.getSpeed())
        }
    }

    // or all who have an attackComponent
    let mut attackComps = manager.borrowComponentVecMut::<AttackDamageComponent>().unwrap();
    let zip = attackComps.iter_mut();
    let iter = zip.filter_map(|attack| Some(attack.as_mut()?));
    for attack in iter{
        println!("someone has an attack damage of {}", attack.getAttackDamage())
    }



    // test::main();

    // let mut manager = Manager{objects:Vec::new()};
    // let mut componentManager = ComponentManager{healthComponents:Vec::new(), healthFreeList:Vec::new()};

    // manager.createObject("RedTower".to_string());

    // let mut go = GameObject{componentIndices:Vec::new()};
    // let go = &mut manager.objects[0].1;

    // go.createHealthComponent(&mut componentManager,65);

    // if componentEnum::HealthComponent(hpComp) = go.getComponent(componentManager, "HealthComponent".to_string()).unwrap() {
    //     println!("{}", hpComp.getHealth());
    // }



    // gameObject::main();
    // assetTest::test();
    // println!("-----");
    // healthComponent::test();
    // println!("-----");
    // moveComponent::test();
    // println!("-----");
    // attackComponent::test();
}
