use crate::NUM_COLS;



pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub alive: bool 
}


pub struct CellGrid {
    pub grid: Vec<Vec<Cell>>,
    pub pubulation: usize,
    pub generation: usize
}

impl CellGrid {
    pub fn new() -> Self {
        let mut grid = Vec::new();
        for x in 0..NUM_COLS {
           // grid.push()
        }
    }
}