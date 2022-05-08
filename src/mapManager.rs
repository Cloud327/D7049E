use std::{path::Path, error::Error, str::FromStr, io::{BufRead, self, BufReader}, fs::File};

use na::{DMatrix, Scalar};
use nalgebra as na;
use crate::tileEnum::TileEnum;


pub struct MapManager{
    mapMatrix: DMatrix<String>
}

impl MapManager{
    pub fn new() -> Self {
        Self {
            // this isnt actually an error its just rust-analyzer being stupid
            mapMatrix: DMatrix::from_element(3,3,"test".to_string())
            // mapMatrix: DMatrix::from_element(15,15,TileEnum::placableTile {})
            // this isnt actually an error its just rust-analyzer being stupid
            // mapMatrix: DMatrix::from_fn(self.mapParser()),
        }
    }

    pub fn mapParser<String>(&mut self) -> Result<i32, Box<dyn Error>>
        // where N: FromStr + Scalar,
        //         N::Err: Error
        {

        let f = File::open("src/resources/map.csv")?;
        let mut reader = BufReader::new(f);

        // initialize an empty vector to fill with numbers
        let mut data = Vec::new();

        // initialize the number of rows to zero; we'll increment this
        // every time we encounter a newline in the input
        let mut rows = 0;

        // for each line in the input,
        for line in reader.lines() {
            // increment the number of rows
            rows += 1;
            // iterate over the items in the row, separated by semicolons
            for datum in line?.split_terminator(";") {
            // trim the whitespace from the item, parse it, and push it to
            // the data array
            data.push((datum.trim().to_string()));
            // data.push(N::from_str(datum.trim())?);
            }
        }

        // The number of items divided by the number of rows equals the
        // number of columns.
        let cols = data.len() / rows;

        // Construct a `DMatrix` from the data in the vector.
        // this isnt actually an error its just rust-analyzer being stupid
        let parsedMap = DMatrix::from_row_slice(rows, cols, &data[..]);
        self.mapMatrix = parsedMap;


        Ok(1)
        }

    pub fn getStart(&self) -> (f32,f32) { // (x,z)

        for column in 0..self.mapMatrix.ncols(){
            for row in 0..self.mapMatrix.nrows(){
                let s = "s".to_string();
                if &self.mapMatrix[(row,column)] == &s {
                // if matches!(self.mapMatrix[(row,column)].getType(), TileEnum::startTile{}) {
                    return (column as f32,row as f32)
                } 
            }

        }

        return (1.0, 1.0)
    }

    pub fn getMapMatrix(&self) -> &DMatrix<String> {
        return &self.mapMatrix
    }

    pub fn findPath(&self) -> Result<Vec<(i32,i32)>,&'static str> { 
        let R:i32 = self.mapMatrix.nrows() as i32;
        let C:i32 = self.mapMatrix.ncols() as i32;
        
        // 0) copy over stuff from M_org to a mutable matrix M (used for keeping track of checked tiles)
        let mut M = DMatrix::from_element(15,15,"g".to_string()); // this is not a error :U
        for r in 0..self.mapMatrix.nrows() {
            for c in 0..self.mapMatrix.ncols() {
                M[(r,c)] = self.mapMatrix[(r,c)].to_string();
            }
        }
    
        // 1) Create BFS queue q and the resulting path
        let mut q:Vec<(i32,i32)> = Vec::new();
        let mut path:Vec<(i32,i32)> = Vec::new();
            
        // 2)scan the matrix
        for i in 0..R{
            for j in 0..C{
                
                // if there exists a cell in the matrix such
                // that its value is "s" (start) then append it to q
                if M[(i as usize, j as usize)] == "s" {
                    q.push((i , j));
                    break
                }
            }
        }
        // 3) run BFS algorithm with q.
        while q.len() != 0 {
            // x = q[0]
            // q = q[1:]
            let x = q.remove(0);
                
            let i = x.0;
            let j = x.1;
                
            // skipping cells which are not valid.
            // if outside the matrix bounds
            if i < 0 || i >= R || j < 0 || j >= C {
                continue
            }
                
            // if they are unpassable "g" (ground) or "t"(tower).
            if M[(i as usize,j as usize)] == "g" || M[(i as usize,j as usize)] == "t"{
                continue
            }
        
            // 3.1) if in the BFS algorithm process there was a
            // vertex x=(i,j) such that M[i][j] is "e" (end) stop and
            // return path
            if M[(i as usize,j as usize)] == "e" {
                path.push((j,i));
                return Ok(path);
            }
                
            // marking as wall upon successful visitation
            // if start or path
            path.push((j,i));
            M[(i as usize,j as usize)] = "g".to_string();
        
            // appending to queue u=(i,j+1),u=(i,j-1)
            // u=(i+1,j),u=(i-1,j)
            q.push((i - 1, j));
            q.push((i, j - 1));
            q.push((i + 1, j));
            q.push((i, j + 1));
            
        }
        // BFS algorithm terminated without returning True
        // then there was no element M[i][j] which is 2, then
        // no available path
        Err("no")
    }
}