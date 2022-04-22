use super::{gameObject::GameObject, healthComponent::HealthComponent, componentEnum::ComponentEnum};


pub struct Manager{
    pub objects: Vec<(String, GameObject)>,
}

impl Manager{
    // Returns object using index, maybe change to id?
    pub fn getObject(&self, index: usize) -> &GameObject{
        return &self.objects[index].1;
    }

    // Destroys object using index, maybe change to id?
    pub fn destroyObject(&mut self, index: usize){
        self.objects.remove(index);
    }

    pub fn createObject(&mut self, objectType: String){
        self.objects.push((objectType, GameObject{componentIndices:Vec::new()}));
    }


}
