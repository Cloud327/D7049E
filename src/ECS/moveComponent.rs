use super::gameObject::BaseComponent;

pub struct MoveComponent{
    speed: usize,
}

impl MoveComponent{
    pub fn new(spd: usize) -> Self {
        Self {
            speed: spd,
        }
    }
    pub fn getSpeed(&self) -> usize{
        return self.speed;
    }

    pub fn setSpeed(&mut self, speed:usize){
        self.speed = speed;
    }
}

impl BaseComponent for MoveComponent{
    fn update(&self) {
        
    }
}

pub fn test(){
    let mut moveComponent = MoveComponent{speed:20};
    println!("Speed at creation: {}",moveComponent.getSpeed());

    moveComponent.setSpeed(30);
    println!("Speed after change: {}",moveComponent.getSpeed());
}