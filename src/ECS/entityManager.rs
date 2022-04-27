use super::{componentVec::ComponentVec};
use std::cell::{RefCell, RefMut};


pub struct EntityManager{
    // We'll use `entities_count` to assign each object a unique ID.
    objectCount: usize,
    componentVecs: Vec<Box<dyn ComponentVec>>,
}

impl EntityManager{
    pub fn new() -> Self {
        Self {
            objectCount: 0,
            componentVecs: Vec::new(),
        }
    }
    /* 
     * Creates a new object with all components set to None
     */
    pub fn newObject(&mut self) -> usize {
        let object_id = self.objectCount;
        for component_vec in self.componentVecs.iter_mut() {
            component_vec.pushNone(); // We call push_none on each component channel because our object will be initialized without any components.
        }

        self.objectCount += 1;
        object_id // And we return object_id so have an index to refer back to later.
    }

    /* 
     * adds a component of specified type to an object specified by its id
     * i think that in this implementation the objects dont actually hold any information
     * and that the info is kept here in the manager where the objects id corresponds to its component
     * in the componentVec Vector component_vecs
     */
    pub fn addComponentToObject<ComponentType: 'static>(
        &mut self,
        object: usize,
        component: ComponentType,
    ) {
        for componentVec in self.componentVecs.iter_mut() {
            if let Some(componentVec) = componentVec
                .asAnyMut()               
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                componentVec.get_mut()[object] = Some(component);
                return;
            }
                
        }
        // No matching component storage exists yet, so we have to make one.
        let mut newComponentVec: Vec<Option<ComponentType>> =
        Vec::with_capacity(self.objectCount);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.objectCount {
            newComponentVec.push(None);
        }

        // Give this object the Component.
        newComponentVec[object] = Some(component);
        self.componentVecs.push(Box::new(RefCell::new(newComponentVec)));

    }
    /* Lets us borrow a componentVec mut
     * Loops through the vector of componentVecs till it finds the one of the right type
     */
    pub fn borrowComponentVecMut<ComponentType: 'static>
    (&self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for componentVec in self.componentVecs.iter() {
            if let Some(componentVec) = componentVec
                .asAny()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(componentVec.borrow_mut());
            }
        }
        None // If we fail to find a matching ComponentVec we simply return None.
    }
    
}




// old



// use super::{gameObject::GameObject, healthComponent::HealthComponent, componentEnum::ComponentEnum};


// pub struct Manager{
//     pub objects: Vec<(String, GameObject)>,
// }

// impl Manager{
//     // Returns object using index, maybe change to id?
//     pub fn getObject(&self, index: usize) -> &GameObject{
//         return &self.objects[index].1;
//     }

//     // Destroys object using index, maybe change to id?
//     pub fn destroyObject(&mut self, index: usize){
//         self.objects.remove(index);
//     }

//     pub fn createObject(&mut self, objectType: String){
//         self.objects.push((objectType, GameObject{componentIndices:Vec::new()}));
//     }


// }
