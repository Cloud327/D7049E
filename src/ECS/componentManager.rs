use super::{healthComponent::HealthComponent, gameObject::GameObject, componentEnum::ComponentEnum};



pub struct ComponentManager{
    pub healthComponents: Vec<HealthComponent>,
    pub healthFreeList:Vec<usize>,
}

impl ComponentManager{

    pub fn createHealthComponent(&mut self, go: &mut GameObject, healthComponent: HealthComponent){
        if self.healthFreeList.is_empty(){
            self.healthComponents.push(healthComponent);
            go.createComponent("HealthComponent".to_string(), self.healthComponents.len()-1);
        }
        else{
            let index = *self.healthFreeList.first().unwrap();
            self.healthComponents.insert(index, healthComponent);
            go.createComponent("HealthComponent".to_string(), index);
        }
    }

    pub fn getComponent(&self, componentType: String, index: usize) -> Option<ComponentEnum>{
        if componentType == "HealthComponent".to_string(){
            return Some(ComponentEnum::healthComponent(self.getHealthComponent(index)));
        }
        else{
            return None;
        }
    }

    pub fn getHealthComponent(&self, index: usize) -> &HealthComponent{
        return &self.healthComponents[index];
    }
    
}