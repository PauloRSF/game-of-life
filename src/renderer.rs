use std::{collections::HashSet, iter::FromIterator};

use terminal_size::{terminal_size, Height, Width};

use crate::{rect::Rect, state::State};

pub struct Renderer {
    viewport: Rect,
    alive_cell_char: char,
    dead_cell_char: char,
}

impl Renderer {
    pub fn new() -> Self {
        let (Width(width), Height(height)) = terminal_size().unwrap();

        Self {
            alive_cell_char: '#',
            dead_cell_char: ' ',
            viewport: Rect {
                bottom_right: (0, 0),
                top_left: (width.into(), height.into()),
            },
        }
    }

    pub fn render_frame(&self, state: &State) {
        print!("{esc}c", esc = 27 as char); // TODO: use some actual terminal UI crate to clear the viewport

        let live_viewport_cells = state
            .get_live_cells()
            .iter()
            .filter(|&cell_location| self.viewport.contains(cell_location));

        let living_cell_indexes = live_viewport_cells
            .map(|point| self.viewport.point_to_index(point))
            .collect::<HashSet<usize>>();

        let data = (0..self.viewport.area()).map(|x| {
            if living_cell_indexes.contains(&x) {
                self.alive_cell_char
            } else {
                self.dead_cell_char
            }
        });

        println!("{}", String::from_iter(data));
    }
}
