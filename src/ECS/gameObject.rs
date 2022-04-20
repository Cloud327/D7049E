

pub struct GameObject{
    
    pub components: Vec<Box<dyn Update>>,
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
    fn update(&self);
    
}
