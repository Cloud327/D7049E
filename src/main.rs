#![allow(non_snake_case)]


mod assetTest;
mod gameManager;
mod meshEnum;
mod meshHandler;
mod ECS;
use ECS::{eventManager, idComponent::IdComponent};
use ECS::attackRateComponent::AttackRateComponent;

use crate::ECS::{entityManager::EntityManager, healthComponent::HealthComponent, moveComponent::MoveComponent,attackDamageComponent::AttackDamageComponent, eventManager::EventManager};

// mod test;

fn main() {

    //eventManager::test();
    gameManager::test();
    /* 
    let mut entManager = EntityManager::new();

    let redEnemy = entManager.newObject();
    entManager.addComponentToObject(redEnemy, HealthComponent{health:65});
    entManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    entManager.addComponentToObject(redEnemy, IdComponent::new(redEnemy));

    let blueEnemy = entManager.newObject();
    entManager.addComponentToObject(blueEnemy, HealthComponent{health:0});
    entManager.addComponentToObject(blueEnemy, MoveComponent::new(2));

    let stationaryEnemy = entManager.newObject();
    entManager.addComponentToObject(stationaryEnemy, HealthComponent{health:0});

    let redTower = entManager.newObject();
    entManager.addComponentToObject(redTower, AttackDamageComponent::new(15));

    let blueTower = entManager.newObject();
    entManager.addComponentToObject(blueTower, AttackRateComponent::new(3));



    // example of how to iterate through all objects who have both a health and movecomponent
    let mut healths = entManager.borrowComponentVecMut::<HealthComponent>().unwrap();
    let mut moves = entManager.borrowComponentVecMut::<MoveComponent>().unwrap();
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
    let mut attackComps = entManager.borrowComponentVecMut::<AttackDamageComponent>().unwrap();
    let zip = attackComps.iter_mut();
    let iter = zip.filter_map(|attack| Some(attack.as_mut()?));
    for attack in iter{
        println!("someone has an attack damage of {}", attack.getAttackDamage())
    } */



    // test::main();

    // let mut entManager = entManager{objects:Vec::new()};
    // let mut componentManager = ComponentManager{healthComponents:Vec::new(), healthFreeList:Vec::new()};

    // entManager.createObject("RedTower".to_string());

    // let mut go = GameObject{componentIndices:Vec::new()};
    // let go = &mut entManager.objects[0].1;

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
