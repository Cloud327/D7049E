

pub trait BaseComponent {
    fn update(&self);

    //fn getComponent(&self);
    
}

pub struct GameObject{

    pub components: Vec<(String, Box::<dyn BaseComponent>)>,

}

impl GameObject {
    pub fn update(&self) {
        
    }

    // -> Option<usize>
    pub fn getComponent(&self, s: String) -> Option<&Box<(dyn BaseComponent + 'static)>>{
        //for component in self.components.iter(){
        //    if(component.as_ref().id == id){
        //        return component;
        //    }
        //}
        //return self.components[0];

        for c in self.components.iter() {
            if c.0 == s {
                return Some(&c.1);
            }
        }
        return None;
        


    }
}
