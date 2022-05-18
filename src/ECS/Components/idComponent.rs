
pub struct IdComponent{
    id: usize
}

impl IdComponent{
    pub fn new(id: usize) -> Self{
        Self {
            id: id,
        }
    }

    pub fn getId(&self) -> usize{
        return self.id
    }

}