
pub struct AttackDamageComponent{
    attackDamage: usize,
}

impl AttackDamageComponent{
    pub fn new(atkDamage: usize) -> Self {
        Self{
            attackDamage: atkDamage,
        }
    }

    pub fn update(){

    }

    pub fn getAttackDamage(&self) -> usize{
        return self.attackDamage;
    }


    pub fn setAttackDamage(&mut self, attackDamage: usize){
        self.attackDamage = attackDamage;
    }
}


pub fn test(){
    let mut attackComponent = AttackDamageComponent{attackDamage:5};
    println!("AttackDamage at creation: {}",attackComponent.getAttackDamage());
    attackComponent.setAttackDamage(10);
    println!("AttackDamage after change: {}", attackComponent.getAttackDamage());
}