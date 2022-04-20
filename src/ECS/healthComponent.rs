use super::gameObject::Update;

struct HealthComponent{
    health: i32,
}

impl HealthComponent{
    pub fn increaseHealth(&mut self, value:i32){
        self.health += value;
    }

    pub fn decreaseHealth(&mut self, value:i32){
        self.health -= value;
    }

    pub fn getHealth (&mut self) -> i32{
        return self.health;
    }
}

impl Update for HealthComponent{
    fn update(&self){

    }
}

pub fn test(){
    let mut healthComponent = HealthComponent{health:50};
    println!("HP at creation: {}",healthComponent.getHealth());
    healthComponent.decreaseHealth(5);

    println!("HP after decrease: {}",healthComponent.getHealth());
    healthComponent.increaseHealth(10);

    println!("HP after increase: {}",healthComponent.getHealth());
}