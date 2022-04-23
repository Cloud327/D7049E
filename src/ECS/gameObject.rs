pub trait BaseComponent {
    fn update(&self);
    
}

pub struct GameObject{
    pub objectID: usize,
    // pub componentIndices: Vec<(String, usize)>,
}

impl GameObject {
    pub fn new(ID: usize) -> Self{
        Self 
        {
            objectID: ID
        }
    }
    pub fn update(&self) {
        
    }
} 







// <-- part of comment

    // pub fn createComponent(&mut self, componentType:String, index:usize){
    //     self.componentIndices.push((componentType, index))
    // }

    // // -> Option<usize>
    // pub fn getComponentIndex(&self, s: String) -> Option<usize>{
    //     for tup in self.componentIndices.iter() {
    //         if tup.0 == s {
    //             return Some(tup.1);
    //         }
    //     }
    //     return None;
    // }

    // pub fn createHealthComponent(&mut self, compManager: &mut ComponentManager, health:usize){
    //     let healthComponent = HealthComponent { health: health };
    //     if compManager.healthFreeList.is_empty(){
    //         compManager.healthComponents.push(healthComponent);
    //         self.createComponent("HealthComponent".to_string(), compManager.healthComponents.len()-1);
    //     }
    //     else{
    //         let index = *compManager.healthFreeList.first().unwrap();
    //         compManager.healthComponents.insert(index, healthComponent);
    //         self.createComponent("HealthComponent".to_string(), index);
    //     }
    // }

    // pub fn getComponent(&self, compManager:ComponentManager, componentType: String) -> Option<ComponentEnum>{
    //     let index = self.getComponentIndex(componentType.clone()).unwrap();
    //     if componentType == "HealthComponent".to_string(){
    //         return Some(ComponentEnum::healthComponent(self.getHealthComponent(compManager, index)));
    //     }
    //     else{
    //         return None;
    //     }
    // }

    // fn getHealthComponent(&self, compManager:ComponentManager,index: usize) -> &HealthComponent{
    //     return compManager.healthComponents.get(index).unwrap();
    // }

// }

