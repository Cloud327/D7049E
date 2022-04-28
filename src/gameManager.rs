use crate::ECS::{eventManager::EventManager, entityManager::EntityManager, healthComponent::HealthComponent, idComponent::IdComponent, eventEnum::EventEnum};


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
            println!("{}, {}", x, y);
        }
    }
}


pub fn test(){
    

    let mut gm = GameManager::new();

    let redEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(redEnemy, HealthComponent{health:65});
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(redEnemy, IdComponent::new(redEnemy));

    let blueEnemy = gm.entityManager.newObject();
    gm.entityManager.addComponentToObject(blueEnemy, HealthComponent{health:90});
    //gm.entityManager.addComponentToObject(redEnemy, MoveComponent::new(1));
    gm.entityManager.addComponentToObject(blueEnemy, IdComponent::new(blueEnemy));

    gm.eventManager.sendEvent(EventEnum::takeDamageEvent { id: 1, damage: 100 });
    gm.eventManager.sendEvent(EventEnum::towerAttackEvent{x: 55, y: 20});
    gm.eventloop();
}