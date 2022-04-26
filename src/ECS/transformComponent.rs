
use nalgebra::Matrix4;

use super::gameObject::BaseComponent;



pub struct TransformComponent{
    pub id: i32,
    pub transform: Matrix4<f32>,

}

impl BaseComponent for TransformComponent {
    fn update(&self) {
        // Jag vet inte
    }

}