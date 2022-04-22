

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
    pub fn getComponentIndex(&self, s: String) -> Option<usize>{
        //for component in self.components.iter(){
        //    if(component.as_ref().id == id){
        //        return component;
        //    }
        //}
        //return self.components[0];

        for tup in self.componentIndices.iter() {
            if tup.0 == s {
                return Some(tup.1);
            }
        }
        return None;
        

    }
}
