use std::path::Path;

use na::{Matrix, ArrayStorage, Const};
use nalgebra as na;

use crate::tileEnum::TileEnum;

pub struct MapManager{
    //mapMatrix: Matrix<TileEnum,Const<15>,Const<15>,ArrayStorage<TileEnum,15,15>>
}

impl MapManager{
    pub fn new() -> Self {
        Self {
            //mapMatrix: Matrix::new(),
        }
    }

    pub fn loadMap(&mut self) {
        
    }

    pub fn getStart() -> (f32,f32) { // (x,z)
        return (1.0, 1.0)
    }

}