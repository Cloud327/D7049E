
pub struct AttackDamageComponent{
    attackDamage: f32,
}

impl AttackDamageComponent{
    pub fn new(attackDamage: f32) -> Self {
        Self{
            attackDamage: attackDamage,
        }
    }

    pub fn update(){

    }

    pub fn getAttackDamage(&self) -> f32{
        return self.attackDamage;
    }


    pub fn setAttackDamage(&mut self, attackDamage: f32){
        self.attackDamage = attackDamage;
    }
}


pub fn test(){
    let mut attackComponent = AttackDamageComponent{attackDamage:5.0};
    println!("AttackDamage at creation: {}",attackComponent.getAttackDamage());
    attackComponent.setAttackDamage(10.0);
    println!("AttackDamage after change: {}", attackComponent.getAttackDamage());
}