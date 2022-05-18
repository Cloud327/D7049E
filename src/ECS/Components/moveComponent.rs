use std::collections::VecDeque;


pub struct MoveComponent{
    speed: f32,
    // Path list containing points in a sequence that represent where the object should move
    path: VecDeque<(i32,i32)>,
}

impl MoveComponent{
    pub fn newWithPath(spd: f32, pth: Vec<(i32, i32)>) -> Self {
        Self {
            speed: spd,
            path: VecDeque::from(pth),
        }
    }
    pub fn new(spd: f32) -> Self {
        Self {
            speed: spd,
            path: VecDeque::new(),
        }
    }

    
    /* Returns the next point to move towards */
    pub fn getNextPoint(&self) -> (f32, f32){
        let point = self.path.front().unwrap();
        return (point.0 as f32, point.1 as f32)
    }

    /* Pops the current next point and returns the new next point in the path list */ 
    // pub fn popAndGetNextPoint(&mut self) -> (f32, f32){
    //     self.path.pop_front();
    //     let point = self.path.front().unwrap();
    //     return (point.0 as f32, point.1 as f32)
    // }
    pub fn popAndGetNextPoint(&mut self) -> Option<(f32, f32)>{
        if self.path.len() == 1 {
            return None
        }
        self.path.pop_front();
        let point = self.path.front().unwrap();
        return Some((point.0 as f32, point.1 as f32))
    }
    
    pub fn getSpeed(&self) -> f32{
        return self.speed;
    }

    pub fn setSpeed(&mut self, speed:f32){
        self.speed = speed;
    }

}



pub fn test(){
    //let mut moveComponent = MoveComponent{speed:20};
    //println!("Speed at creation: {}",moveComponent.getSpeed());

    //moveComponent.setSpeed(30);
    //println!("Speed after change: {}",moveComponent.getSpeed());
}