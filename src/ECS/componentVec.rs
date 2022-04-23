use std::cell::RefCell;
/* 
 * This is a vector holding components of a single type
 * Implementation is generic so it can hold any of our different components 
*/
pub trait ComponentVec {
    fn asAny(&self) -> &dyn std::any::Any;
    fn asAnyMut(&mut self) -> &mut dyn std::any::Any;
    fn pushNone(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn asAny(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn asAnyMut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn pushNone(&mut self) {
        self.get_mut().push(None)
    }
}