use super::gameObject::BaseComponent;

struct MoveComponent{
    speed: i32,
}

impl MoveComponent{
    pub fn getSpeed(&self) -> i32{
        return self.speed;
    }

    pub fn setSpeed(&mut self, speed:i32){
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