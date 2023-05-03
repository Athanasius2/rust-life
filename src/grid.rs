pub mod grid {

use std::collections::BTreeSet;
use std::collections::BTreeMap;

pub type Cell = (i32, i32); //will switch to generics later, but it's too painful for now

pub struct Grid{
    cells: BTreeSet<Cell>
}

impl Grid{
    /// Takes a cell coordinate and returns an array of the 8 neighboring cell coordinates.
    /// 
    /// ### Arguments
    /// 
    /// * `cell` - Input cell coordinates
    /// 
    /// ### Return \[Cell; 8\]
    /// 
    /// Array of coordinates for the cells surrounding `cell`
    fn get_neighbors(&self, cell: Cell) -> [Cell; 8] {
        [
            (cell.0 - 1, cell.1 - 1),
            (cell.0 - 1, cell.1),
            (cell.0 - 1, cell.1 + 1),
            
            (cell.0, cell.1 - 1),
            (cell.0, cell.1 + 1),
            
            (cell.0 + 1, cell.1 - 1),
            (cell.0 + 1, cell.1),
            (cell.0 + 1, cell.1 + 1),
        ]
    }

    /// Finds the number of living cells neighboring each cell in the Grid.
    /// 
    /// This function iterates through Grid.cells, gets coordinates of the 8 neighbors 
    /// of each cell and adds them to a Vec. Each instance of a coordinate (x, y) 
    /// represents one living cell adjacent to (x, y).
    /// 
    /// ### Return Vec\<Cell\>
    /// 
    /// Returns a vector of coordinates representing the number of living cells adjacent
    /// to each coordinate.
    fn count_live_neighbors(&self) -> Vec<Cell> {
        self.cells
            .clone()
            .iter()
            .map( |cell| self.get_neighbors(*cell) )
            .flatten()
            .collect()
    }

    fn reducer(&self, neighbor_counter: Vec<Cell>) -> BTreeMap<Cell, i8> {
        let mut reduced: BTreeMap<Cell, i8> = BTreeMap::new();
        
        neighbor_counter.iter().for_each(|cell| {
            if let Some(cell_count) = reduced.get_mut(cell) {
                *cell_count += 1;
            } else {
                reduced.insert(*cell, 1);
            }
        });

        reduced
    }

    pub fn from_b_tree_set(_cells: BTreeSet<Cell>) -> Self {
        Grid { cells: _cells }
    }

    pub fn new () -> Self {
        Grid { cells: BTreeSet::new() }
    }

    pub fn get_cells(&self) -> BTreeSet<Cell> {
        self.cells.clone()
    }
    
    pub fn clear_cells(&mut self) {
        self.cells.clear();
    }

    pub fn toggle_cell(&mut self, cell: Cell) {
        if !self.cells.insert(cell) { self.cells.remove(&cell); } 
    }

    pub fn compute(&mut self) {
        let neighbor_counter = self.count_live_neighbors();
        let reduced = self.reducer(neighbor_counter);

        // Remove cells with less than 2 or more than 3 living neighbors.
        self.cells =
            reduced.iter()
                .filter(|(cell_coordinates, cell_neighbors)| { 
                    (cell_neighbors == &&2 && self.cells.contains(&cell_coordinates)) || cell_neighbors == &&3
                })
                .map(|(key, _)| *key )
                .collect(); 

        //println!("{:?}", neighbor_counter);
        //println!("{:?}", reduced);
        //println!("{:?}", self.cells);
    }

    pub fn iterative_compute(&mut self, iterations: i32) {
        (0..iterations).for_each( |_| self.compute());
    }
}

}