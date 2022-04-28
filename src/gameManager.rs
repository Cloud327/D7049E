use crate::ECS::{entityManager::EntityManager, eventManager::EventManager, eventEnum::EventEnum};



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

    pub fn gameloop(self){
        self.eventManager.update();
    }

    fn eventloop(&mut self){
        let event = self.eventManager.readEvent();

        if let EventEnum::takeDamageEvent{id, damage} = event {
            
            // find HealthComponent on index=id
            //
        }

        if let EventEnum::towerAttackEvent{x, y} = event {
            println!("{}, {}", x, y);
        }
    }
}


pub fn test(){
    

    



}