

pub trait BaseComponent {
    fn update(&self);

    fn getComponent(&self);
    
}

pub struct GameObject{
    


    pub components: Vec<(&'static str, Box::<dyn BaseComponent>)>,
    //pub componentIndex: Vec<(&'static str, usize)>
    //pub components: Vec<(&'static str, usize)>,
}

impl GameObject {
    pub fn update(&self) {
        
    }

    // -> Option<usize>
    pub fn getComponent(&self, s: &str) -> Option<Box<(dyn BaseComponent + 'static)>>{
        //for component in self.components.iter(){
        //    if(component.as_ref().id == id){
        //        return component;
        //    }
        //}
        //return self.components[0];

        for c in self.components.iter() {
            if c.0 == s {
                return Some(c.1);
            }
        }
        return None;
        

        /*for component in self.components.iter() {
            if component.0 == s {
                return Some(component.1);
            }
        }
        return None*/

    }
}


