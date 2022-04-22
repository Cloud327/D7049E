use super::{componentManager::ComponentManager, healthComponent::HealthComponent, componentEnum::ComponentEnum};



pub trait BaseComponent {
    fn update(&self);
    
}

pub struct GameObject{

    pub componentIndices: Vec<(String, usize)>,

}

impl GameObject {
    pub fn update(&self) {
        
    }

    pub fn createComponent(&mut self, componentType:String, index:usize){
        self.componentIndices.push((componentType, index))
    }

    // -> Option<usize>
    fn getComponentIndex(&self, s: String) -> Option<usize>{
        for tup in self.componentIndices.iter() {
            if tup.0 == s {
                return Some(tup.1);
            }
        }
        return None;
    }

    pub fn createHealthComponent(&mut self, &mut compManager: &mut ComponentManager, healthComponent: HealthComponent){
        if compManager.healthFreeList.is_empty(){
            compManager.healthComponents.push(healthComponent);
            self.createComponent("HealthComponent".to_string(), compManager.healthComponents.len()-1);
        }
        else{
            let index = *compManager.healthFreeList.first().unwrap();
            compManager.healthComponents.insert(index, healthComponent);
            self.createComponent("HealthComponent".to_string(), index);
        }
    }

    pub fn getComponent(&self, compManager:ComponentManager, componentType: String) -> Option<ComponentEnum>{
        let index = self.getComponentIndex(componentType).unwrap();
        if componentType == "HealthComponent".to_string(){
            return Some(ComponentEnum::healthComponent(*self.getHealthComponent(compManager, index).unwrap()));
        }
        else{
            return None;
        }
    }

    fn getHealthComponent(&self, compManager:ComponentManager,index: usize) -> Option<&HealthComponent>{
        return compManager.healthComponents.get(index);
    }

    
}
