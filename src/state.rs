use std::collections::HashSet;

use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};
use terminal_size::{terminal_size, Height, Width};

use crate::rect::Point;

#[derive(Clone, Debug)]
pub struct State {
    pub live_cells: HashSet<Point>,
}

impl State {
    pub fn new() -> Self {
        Self {
            live_cells: HashSet::new(),
        }
    }

    pub fn new_randomized(live_cell_count: usize) -> Self {
        let (Width(width), Height(height)) = terminal_size().unwrap();

        let mut cells = (0..width as i32)
            .cartesian_product(0..height as i32)
            .collect::<Vec<Point>>();

        cells.shuffle(&mut thread_rng());

        let live_cells = cells
            .iter()
            .cloned()
            .take(live_cell_count)
            .collect::<HashSet<Point>>();

        Self { live_cells }
    }

    pub fn get_live_cells(&self) -> &HashSet<(i32, i32)> {
        &self.live_cells
    }

    pub fn put_cell(&mut self, x: i32, y: i32) {
        self.live_cells.insert((x, y));
    }

    pub fn cell_neighborhood(x: i32, y: i32) -> [Point; 8] {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    }

    fn cell_should_live(&self, cell: &Point, neighbors: usize) -> bool {
        let is_cell_alive = self.live_cells.contains(cell);
        neighbors == 3 || (is_cell_alive && neighbors == 2)
    }

    pub fn next_state(&self) -> Self {
        let cells_with_neighbor_count = &self
            .live_cells
            .iter()
            .map(|&(x, y)| State::cell_neighborhood(x, y))
            .flatten()
            .counts();

        let surviving_cells = cells_with_neighbor_count
            .iter()
            .filter(|&(cell, &neighbors)| self.cell_should_live(&cell, neighbors))
            .map(|(&val, _)| val)
            .collect();

        Self {
            live_cells: surviving_cells,
        }
    }
}
