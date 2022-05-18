
pub struct AttackRateComponent{
    attackRate: f32,
}

impl AttackRateComponent{
    pub fn new(atkRate: f32) -> Self {
        Self{
            attackRate: atkRate,
        }
    }
    
    pub fn getAttackRate(&self) -> f32{
        return self.attackRate;
    }

    pub fn setAttackRate(&mut self, attackRate: f32){
        self.attackRate = attackRate;
    }
}


pub fn test(){
    let mut attackComponent = AttackRateComponent{attackRate:1.0};
    println!("AttackRate at creation: {}",attackComponent.getAttackRate());
    attackComponent.setAttackRate(2.0);
    println!("AttackRate after change: {}",attackComponent.getAttackRate());
}