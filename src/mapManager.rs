use std::{error::Error,io::{BufRead, BufReader}, fs::File};

use kiss3d::window::Window;
use na::{DMatrix, Translation3};
use nalgebra as na;
use rand::Rng;

pub struct MapManager{
    mapMatrix: DMatrix<String>
}

impl MapManager{
    pub fn new() -> Self {
        Self {
            // this isnt actually an error its just rust-analyzer being stupid
            mapMatrix: DMatrix::from_element(3,3,"test".to_string())
        }
    }
    
    /*
     * Reads the map.csv file in resources and constructs our mapMatrix based on it
     * map.csv should contain only the strings s,e,r,g,t for start, end, road, ground, tower
     * seperated by semicolons since that is what excel gave me
     */
    pub fn parseMap(&mut self, mapPath: &str) -> Result<i32, Box<dyn Error>> {

        let f = File::open(mapPath)?;
        let reader = BufReader::new(f);

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
            data.push(datum.trim().to_string());
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


    /* 
     * Finds and returns the position of the start tile in the mapMatrix
     * by looping through the mapMatrix... 
     */
    pub fn getStart(&self) -> (f32,f32) { // (x,z)
        for column in 0..self.mapMatrix.ncols(){
            for row in 0..self.mapMatrix.nrows(){
                if &self.mapMatrix[(row,column)] == "s" {
                    return (column as f32 + 100.0 ,row as f32 + 100.0)
                } 
            }
        }
        return (100.0, 100.0)
    }

    /* 
     * Finds and returns the position of the end tile in the mapMatrix
     * by looping through the mapMatrix... 
     */
    pub fn getEnd(&self) -> (f32,f32) { // (x,z)
        for column in 0..self.mapMatrix.ncols(){
            for row in 0..self.mapMatrix.nrows(){
                if &self.mapMatrix[(row,column)] == "e" {
                    return (column as f32 + 100.0 ,row as f32 + 100.0)
                } 
            }
        }
        return (100.0, 100.0)
    }

    /*
     * returns the mapMatrix
     */
    pub fn getMapMatrix(&self) -> &DMatrix<String> {
        return &self.mapMatrix
    }


    /*
     * Implements a BFS to find a path from start (s) to end (e) using only road-tiles (r)
     * returns the path as an ordered vector of tuples
     */
    pub fn findPath(&self) -> Result<Vec<(i32,i32)>,&'static str> { 
        let R:i32 = self.mapMatrix.nrows() as i32;
        let C:i32 = self.mapMatrix.ncols() as i32;
        
        // 0) copy over stuff from M_org to a mutable matrix M (used for keeping track of checked tiles)
        let mut M = DMatrix::from_element(R as usize,C as usize,"g".to_string()); // this is not a error :U
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
                path.push((j+100,i+100));
                return Ok(path);
            }
                
            // marking as wall upon successful visitation
            // if start or path
            path.push((j+100,i+100));
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
        return Err("no");
    }



    /*
     * Generates a renderable map from the data in the mapMatrix
     */
    pub fn drawMap(&self, window: &mut Window) {
        // makes if cases below clearer
        let roadTiles = "rse";
        let groundTiles = "gt";

        
        // loop thru mapMatrix
        for row in 0..self.mapMatrix.nrows(){
            for col in 0..self.mapMatrix.ncols(){
                // what do i do here?
                // depending of character in mm[(r,c)] draw a tile in a different color?
                let currentTile = &self.mapMatrix[(row,col)];
                if groundTiles.contains(currentTile) {
                    let mut tile = window.add_cube(1.0,0.2,1.0);
                    tile.set_local_translation(Translation3::new(col as f32+100.0, 0.0, row as f32+100.0));
                    tile.set_color(0.2, 0.6, 0.0); // ground is green
                } else if roadTiles.contains(currentTile) {
                    let mut tile = window.add_cube(1.0,0.15,1.0);
                    tile.set_local_translation(Translation3::new(col as f32+100.0, 0.0, row as f32+100.0));
                    tile.set_color(0.8, 0.4, 0.0); // road is brown
                } 
            } 
        } 
    }



    /* 
     * Finds and returns the position of all tiles with towers in the mapMatrix
     * by looping through the mapMatrix... 
     */
    pub fn getTowerLocations(&self)  -> Vec<(f32,f32)> {
        let mut towerList: Vec<(f32,f32)> = Vec::new();

        for column in 0..self.mapMatrix.ncols(){
            for row in 0..self.mapMatrix.nrows(){
                if &self.mapMatrix[(row,column)] == "t" {
                    towerList.push((column as f32+100.0,row as f32+100.0));
                } 
            }
        }
        return towerList;
    }


    /* 
     * Finds and returns the position of all tiles with towers in the mapMatrix
     * by looping through the mapMatrix... 
     */
    pub fn nextTowerLocation(&mut self)  -> Result<(f32,f32),&'static str> {
        let mut freeList: Vec<(f32,f32)> = Vec::new();

        for column in 0..self.mapMatrix.ncols(){
            for row in 0..self.mapMatrix.nrows(){
                if &self.mapMatrix[(row,column)] == "g" {
                    freeList.push((column as f32,row as f32));
                } 
            }
        }

        if freeList.len() > 0 {
            let mut nextTower = freeList[rand::thread_rng().gen_range(0..freeList.len())];
            self.mapMatrix[(nextTower.1 as usize,nextTower.0 as usize)] = "t".to_string();
            nextTower.0 += 100.0;
            nextTower.1 += 100.0;
            return Ok(nextTower);
        }
        return Err("no free tiles");
    }
}