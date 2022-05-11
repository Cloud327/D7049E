use super::{eventEnum::{EventEnum}};


pub struct EventManager{
    eventBuffer: Vec<EventEnum>,
}

impl EventManager{
    pub fn new() -> Self {
        Self {
            eventBuffer: Vec::new()
        }
    }

    pub fn sendEvent(&mut self, event: EventEnum){
        self.eventBuffer.push(event);
    }

    pub fn readEvent(&mut self) -> EventEnum{
        return self.eventBuffer.remove(0)
    }


}



pub fn test(){
    let mut em = EventManager::new();
    em.sendEvent(EventEnum::takeDamageEvent { id: 0, damage: 22 });
    em.sendEvent(EventEnum::towerAttackEvent{xTarget: 55, yTarget: 20, zTarget: 2});

    let t = em.readEvent();

    if let EventEnum::takeDamageEvent{id, damage} = t {
        println!("{}, {}", id, damage);
    }

    let t = em.readEvent();

    if let EventEnum::towerAttackEvent{xTarget, yTarget, zTarget} = t {
        println!("{}, {}", xTarget, yTarget);
    }



    

}