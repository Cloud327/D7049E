use std::collections::HashMap;


pub struct GameObject{
    pub components: HashMap<String, Box<dyn BaseComponent>>,
    //pub components: Vec<Box<dyn Update>>,
}

impl GameObject {
    pub fn update(&self) {
        // Update
    }

    pub fn getComponent(&self, componentType: String) -> &Box<dyn BaseComponent>{
        //Somehow get component
        let component = self.components.get(&componentType);
        return component.unwrap();
    }
}


pub trait BaseComponent{
    fn update(&self);
    
}

