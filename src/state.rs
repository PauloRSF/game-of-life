use std::collections::HashSet;

use itertools::Itertools;

use crate::Cell;

#[derive(Clone, Debug)]
pub struct State {
    pub live_cells: HashSet<Cell>,
}

impl From<&[Cell]> for State {
    fn from(cells: &[Cell]) -> Self {
        Self {
            live_cells: cells.iter().cloned().collect::<_>(),
        }
    }
}

impl State {
    fn cell_neighborhood(x: i32, y: i32) -> [Cell; 8] {
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

    fn cell_should_live(&self, cell: &Cell, neighbors: usize) -> bool {
        let is_cell_alive = self.live_cells.contains(cell);

        neighbors == 3 || (is_cell_alive && neighbors == 2)
    }

    pub fn next(&self) -> Self {
        let cells_with_neighbor_count = &self
            .live_cells
            .iter()
            .flat_map(|&(x, y)| State::cell_neighborhood(x, y))
            .counts();

        let surviving_cells = cells_with_neighbor_count
            .iter()
            .filter(|&(cell, &neighbors)| self.cell_should_live(cell, neighbors))
            .map(|(&val, _)| val)
            .collect();

        Self {
            live_cells: surviving_cells,
        }
    }
}
