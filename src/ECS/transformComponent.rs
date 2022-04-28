pub struct TransformComponent{
    transform: Matrix4<usize> // Ska det verkligen vara usize? Kanske behövs floats

}

impl TransformComponent{
    pub fn new(transform: Matrix4<usize>)-> Self{
        Self{
            transform:transform
        }
    }

    pub fn update(){

    }


}