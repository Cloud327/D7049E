use super::gameObject::GameObject;



pub struct Manager{
    objects: Vec<(usize, String, GameObject)>,
}

impl Manager{

    // Returns object using index, maybe change to id?
    pub fn getObject(&self, index: usize) -> &GameObject{
        return &self.objects[index].2;
    }

    // Destroys object using index, maybe change to id?
    pub fn destroyObject(&mut self, index: usize){
        self.objects.remove(index);
    }


}