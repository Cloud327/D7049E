

pub struct GameObject{
    
    pub components: vec<Box<dyn Update>>,
}

impl GameObject {
    pub fn update(&self) {
        // Update
    }

    pub fn getComponent(){
        //Somehow get component
    }
}


pub trait Update {
    pub fn update(&self);
    
}
