use super::gameObject::BaseComponent;

pub struct AttackRateComponent{
    attackRate: usize,
}

impl AttackRateComponent{
    pub fn new(atkRate: usize) -> Self {
        Self{
            attackRate: atkRate,
        }
    }
    pub fn getAttackRate(&self) -> usize{
        return self.attackRate;
    }

    pub fn setAttackRate(&mut self, attackRate: usize){
        self.attackRate = attackRate;
    }
}

impl BaseComponent for AttackRateComponent{
    fn update(&self) {
        
    }
}

pub fn test(){
    let mut attackComponent = AttackRateComponent{attackRate:1};
    println!("AttackRate at creation: {}",attackComponent.getAttackRate());
    attackComponent.setAttackRate(2);
    println!("AttackRate after change: {}",attackComponent.getAttackRate());
}