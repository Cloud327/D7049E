use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum, 
    typeEnum::TypeEnum, typeComponent::TypeComponent, attackDamageComponent::AttackDamageComponent};


pub struct GameManager{
    entityManager: EntityManager,
    eventManager: EventManager,
}


impl GameManager{
    pub fn new() -> Self {
        Self {
            entityManager: EntityManager::new(),
            eventManager: EventManager::new(),
        }
    }

    pub fn initialize(){

    }

    pub fn gameloop(&self){
        
    }

    fn eventloop(&mut self){
        let event = self.eventManager.readEvent();

        if let EventEnum::takeDamageEvent{id, damage} = event {
            let mut healthComp = self.entityManager.borrowComponentVecMut::<HealthComponent>().unwrap();
            let mut idComp = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = healthComp.iter_mut().zip(idComp.iter_mut());

            let iter = zip.filter_map(|(health, idThing)| Some((health.as_mut()?, idThing.as_mut()?)));
            for (health, idThing) in iter {
                if idThing.getId() == id {
                    println!("health {} at id: {}", health.getHealth(), id);
                    health.decreaseHealth(damage);
                    println!("health {} at id: {}", health.getHealth(), id);
                } else {
                    println!("No such id");
                }
            }
        }

        if let EventEnum::towerAttackEvent{x, y} = event {
            // Do attack with all object of type = towers
            // Do attack = create projectile object
            let mut typeComp = self.entityManager.borrowComponentVecMut::<TypeComponent>().unwrap();
            let mut idComp = self.entityManager.borrowComponentVecMut::<IdComponent>().unwrap();
            let zip = typeComp.iter_mut().zip(idComp.iter_mut());

            let iter = zip.filter_map(|(typeThing, idThing)| Some((typeThing.as_mut()?, idThing.as_mut()?)));
            for (typeThing, idThing) in iter {
                if matches!(typeThing.getType(), TypeEnum::towerType{}){
                    println!("found tower at id: {}", idThing.getId());
                    // Do attack
                }

            }
            println!("{}, {}", x, y);
        }
    }
}


pub fn test(){
    

    let mut gm = GameManager::new();

    let redEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(redEnemy, HealthComponent::new(65));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(redEnemy, IdComponent::new(redEnemy));
    gm.entityManager.addComponentToObject(redEnemy, TypeComponent::new(TypeEnum::enemyType { }));


    let whiteTower = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(whiteTower, AttackDamageComponent::new(8));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(whiteTower, IdComponent::new(whiteTower));
    gm.entityManager.addComponentToObject(whiteTower, TypeComponent::new(TypeEnum::towerType { }));


    let blueEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(blueEnemy, HealthComponent::new(90));
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(blueEnemy, IdComponent::new(blueEnemy));
    gm.entityManager.addComponentToObject(blueEnemy, TypeComponent::new(TypeEnum::enemyType { }));


    gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20});
    gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 2, damage: 10 });
    

    gm.eventloop();
    gm.eventloop();
}