use super::gameObject::BaseComponent;

struct AttackComponent{
    attackRate: i32,
    attackDamage: i32,
}

impl AttackComponent{
    pub fn getAttackRate(&self) -> i32{
        return self.attackRate;
    }

    pub fn getAttackDamage(&self) -> i32{
        return self.attackDamage;
    }

    pub fn setAttackRate(&mut self, attackRate: i32){
        self.attackRate = attackRate;
    }

    pub fn setAttackDamage(&mut self, attackDamage: i32){
        self.attackDamage = attackDamage;
    }
}

impl BaseComponent for AttackComponent{
    fn update(&self) {
        
    }
}

pub fn test(){
    let mut attackComponent = AttackComponent{attackRate:1, attackDamage:5};
    println!("AttackRate and AttackDamage at creation: {}, {}",attackComponent.getAttackRate(), attackComponent.getAttackDamage());
    attackComponent.setAttackRate(2);
    attackComponent.setAttackDamage(10);
    println!("AttackRate and AttackDamage after change: {}, {}",attackComponent.getAttackRate(), attackComponent.getAttackDamage());
}