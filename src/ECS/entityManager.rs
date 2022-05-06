use super::{componentVec::ComponentVec, idComponent::IdComponent};
use std::{cell::{RefCell, RefMut}};


pub struct EntityManager{
    // We'll use `entities_count` to assign each object a unique ID.
    currentID: usize,
    componentVecs: Vec<Box<dyn ComponentVec>>,
}

impl EntityManager{
    pub fn new() -> Self {
        Self {
            currentID: 0,
            componentVecs: Vec::new(),
        }
    }



    /* 
     * Creates a new object with all components set to None
     */
    pub fn newObject(&mut self) -> usize {
        let object_id = self.currentID;
        for component_vec in self.componentVecs.iter_mut() {
            component_vec.pushNone(); // We call push_none on each component channel because our object will be initialized without any components.
        }
        // add the IDcomponent... >:I
        self.addIdComponentToObject(object_id);

        self.currentID += 1;
        return object_id; // And we return object_id so have an index to refer back to later.
    }



    /*
     * Removes an object by setting all its components to None
     */
    pub fn removeObject(&mut self, idToRemove:usize) {
        // step 1: find index of idToRemove
        let mut idCompList = self.borrowComponentVecMut::<IdComponent>().unwrap();
        let iter = idCompList.iter_mut().filter_map(|id| Some(id.as_mut()?));
        let mut target = 0; // use this to find the targetIds index in the idCompList
        for id in iter{
            if id.getId() == idToRemove{
                break;
            }
            target += 1;
        }
        drop(idCompList);

        // step 2: remove at index
        for componentVec in self.componentVecs.iter_mut() {
            componentVec.removeAt(target); 
        }
    }



    /* 
     * adds an idComponent to a object, similar to addComponentToObject but here we cant use 
     * an objects idComponent since it doesnt exist yet.
     */
    pub fn addIdComponentToObject( //<ComponentType: 'static>(
        &mut self,
        object: usize,
        // component: ComponentType,
    ) {
        // add component to the correct (if already existing) componentVec
        for componentVec in self.componentVecs.iter_mut() { 
            if let Some(componentVec) = componentVec
                .asAnyMut()               
                .downcast_mut::<RefCell<Vec<Option<IdComponent>>>>()
            {
                componentVec.get_mut()[object] = Some(IdComponent::new(object));
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut newComponentVec: Vec<Option<IdComponent>> =
        Vec::with_capacity(self.componentVecs.len()+1);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.componentVecs.len()+1 {
            newComponentVec.push(None);
        }

        // Give this object the Component.
        newComponentVec[object] = Some(IdComponent::new(object));
        self.componentVecs.push(Box::new(RefCell::new(newComponentVec)));
    }



    /* 
     * adds a component of specified type to an object specified by its id
     * i think that in this implementation the objects dont actually hold any information
     * and that the info is kept here in the manager where the objects id corresponds to its component
     * in the componentVec Vector component_vecs
     */
    pub fn addComponentToObject<ComponentType: 'static>(
        &mut self,
        targetId: usize,
        component: ComponentType,
    ) {
        let mut idCompList = self.borrowComponentVecMut::<IdComponent>().unwrap();
        let iter = idCompList.iter_mut().filter_map(|id| Some(id.as_mut()?));
        let mut target = 0; // use this to find the targetIds index in the idCompList
        for id in iter{
            if id.getId() == targetId{
                break;
            }
            target += 1;
        }
        drop(idCompList);
        
        // add component to the correct (if already existing) componentVec
        for componentVec in self.componentVecs.iter_mut() { 
            if let Some(componentVec) = componentVec
                .asAnyMut()               
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                componentVec.get_mut()[target] = Some(component);
                return;
            }
        }
        // No matching component storage exists yet, so we have to make one.
        let mut newComponentVec: Vec<Option<ComponentType>> =
        Vec::with_capacity(self.componentVecs.len());

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.componentVecs.len() {
            newComponentVec.push(None);
        }

        // Give this object the Component.
        newComponentVec[target] = Some(component);
        self.componentVecs.push(Box::new(RefCell::new(newComponentVec)));

    }



    /* Lets us borrow a componentVec mut
     * Loops through the vector of componentVecs till it finds the one of the right type
     */
    pub fn borrowComponentVecMut<ComponentType: 'static> (&self, ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
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