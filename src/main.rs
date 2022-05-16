#![allow(non_snake_case)]

use crate::ECS::{entityManager::EntityManager, Components::{healthComponent::HealthComponent, attackRateComponent::AttackRateComponent, idComponent::IdComponent, attackDamageComponent::AttackDamageComponent}};

// mod assetTest;
// mod buttonTest;
mod gameManager;
mod nodeHandler;
mod physicsManager;
// mod tileEnum;
mod mapManager;
mod ECS;
mod colliderTest;
mod gameStateEnum;


fn main() {

    // let mut vec: Vec<Option<i32>> = Vec::new();
    // vec.push(Some(11));
    // vec.push(None);
    // vec.push(Some(13));


    // let a = [1,2,3].iter();
    // let b = [4,5,6].iter();
    // let c = ['a','b','c'].iter();
    // let d = ["d","NaN","f"].iter();
    // // let d = vec.iter();
    // let zip = a.zip(b.zip(c.zip(d)));

    // let iter = zip.filter_map(|(a, (b, (c, d))),
    //                                                         |Some((a, b, c, d)));
        
    // for (a,b,c,d) in iter {
    //     println!("{}, {}, {}, {}", a, b, c , d);
    // }

    // ===============

    // iterTest();
    // removeTest();
    gameManager::test();

}
fn iterTest() {

    let mut em = EntityManager::new();
    let o0 = em.newObject();
    let o1 = em.newObject();
    let o2 = em.newObject();
    let o3 = em.newObject();

    em.addComponentToObject(o0, HealthComponent::new(0));
    em.addComponentToObject(o1, HealthComponent::new(1));
    em.addComponentToObject(o2, HealthComponent::new(2));
    em.addComponentToObject(o3, HealthComponent::new(3));

    em.addComponentToObject(o0, AttackRateComponent::new(0.0));
    em.addComponentToObject(o1, AttackRateComponent::new(1.0));
    em.addComponentToObject(o2, AttackRateComponent::new(2.0));
    em.addComponentToObject(o3, AttackRateComponent::new(3.0));

    em.addComponentToObject(o0, AttackDamageComponent::new(0.0));
    em.addComponentToObject(o1, AttackDamageComponent::new(1.0));
    em.addComponentToObject(o2, AttackDamageComponent::new(2.0));
    em.addComponentToObject(o3, AttackDamageComponent::new(3.0));


    let mut idList = em.borrowComponentVecMut::<IdComponent>().unwrap();
    let mut hpList = em.borrowComponentVecMut::<HealthComponent>().unwrap();
    let mut arList = em.borrowComponentVecMut::<AttackRateComponent>().unwrap();
    let mut adList = em.borrowComponentVecMut::<AttackDamageComponent>().unwrap();

    let zip = idList.iter_mut().zip(hpList.iter_mut().zip(arList.iter_mut().zip(adList.iter_mut())));

    let iter = zip.filter_map(|(id, (hp, (ar, ad))),
        |Some((id.as_mut()?, hp.as_mut()?, ar.as_mut()?, ad.as_mut()?)));

    for (id,hp,ar,ad) in iter {
        println!("id: {}, hp: {}, ar: {}, ad: {}", id.getId(), hp.getHealth(), ar.getAttackRate() , ad.getAttackDamage());
    }

    drop(idList);
    drop(hpList);
    drop(arList);
    drop(adList);

    println!("dont mind me im just removing stuff");
    em.removeObject(o3);

    let mut idList = em.borrowComponentVecMut::<IdComponent>().unwrap();
    let mut hpList = em.borrowComponentVecMut::<HealthComponent>().unwrap();
    let mut arList = em.borrowComponentVecMut::<AttackRateComponent>().unwrap();
    let mut adList = em.borrowComponentVecMut::<AttackDamageComponent>().unwrap();

    let zip = idList.iter_mut().zip(hpList.iter_mut().zip(arList.iter_mut().zip(adList.iter_mut())));

    let iter = zip.filter_map(|(id, (hp, (ar, ad))),
        |Some((id.as_mut()?, hp.as_mut()?, ar.as_mut()?, ad.as_mut()?)));

    for (id,hp,ar,ad) in iter {
        println!("id: {}, hp: {}, ar: {}, ad: {}", id.getId(), hp.getHealth(), ar.getAttackRate() , ad.getAttackDamage());
    }
}






fn removeTest(){
    let mut em = EntityManager::new();
    let o1 = em.newObject();
    let o2 = em.newObject();
    let o3 = em.newObject();
    let o4 = em.newObject();
    

    em.addComponentToObject(o1, HealthComponent::new(0));
    em.addComponentToObject(o2, HealthComponent::new(1));
    em.addComponentToObject(o3, HealthComponent::new(2));
    em.addComponentToObject(o4, HealthComponent::new(3));

    em.addComponentToObject(o1, AttackRateComponent::new(0.0));
    em.addComponentToObject(o2, AttackRateComponent::new(1.0));
    em.addComponentToObject(o3, AttackRateComponent::new(2.0));
    em.addComponentToObject(o4, AttackRateComponent::new(3.0));

    let mut idCompList = em.borrowComponentVecMut::<IdComponent>().unwrap();
    let iter = idCompList.iter_mut().filter_map(|id| Some(id.as_mut()?));

    for id in iter {
        println!("located object with id {}", id.getId());
    }
    drop(idCompList);

    em.removeObject(o2);
    em.removeObject(o4);
    println!("removing stuff :)");

    let mut arCompList = em.borrowComponentVecMut::<AttackRateComponent>().unwrap();
    let iter = arCompList.iter_mut().filter_map(|ar| Some(ar.as_mut()?));

    for ar in iter {
        println!("located object with attack rate {}", ar.getAttackRate());
    }

    let mut idCompList = em.borrowComponentVecMut::<IdComponent>().unwrap();
    let iter = idCompList.iter_mut().filter_map(|id| Some(id.as_mut()?));

    for id in iter {
        println!("located object with id {}", id.getId());
    }
}