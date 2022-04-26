use super::{healthComponent::HealthComponent, gameObject::GameObject, componentEnum::ComponentEnum};



pub struct ComponentManager{
    pub healthComponents: Vec<HealthComponent>,
    pub healthFreeList:Vec<usize>,
}

impl ComponentManager{
    
}