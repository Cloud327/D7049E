

pub struct HealthComponent{
    pub health: usize,
}

impl HealthComponent{
    pub fn new(health: usize) -> Self{
        Self {
            health: health
        }
    }

    pub fn update(){

    }
    
    pub fn increaseHealth(&mut self, value:usize){
        self.health += value;
    }

    pub fn decreaseHealth(&mut self, value:usize){
        self.health -= value;
    }

    pub fn getHealth (&mut self) -> usize{
        return self.health;
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