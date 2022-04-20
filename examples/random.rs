use std::io;

use itertools::Itertools;
use rand::{prelude::SliceRandom, thread_rng};
use terminal_size::{terminal_size, Height, Width};

use game_of_life::{
    runner::{self, RunnerOptions},
    state::State,
    Cell,
};

fn main() -> io::Result<()> {
    let (Width(width), Height(height)) = terminal_size().unwrap();

    let mut cells = (0..width as i32)
        .cartesian_product(0..height as i32)
        .collect::<Vec<Cell>>();

    cells.shuffle(&mut thread_rng());

    let live_cells = cells
        .iter()
        .cloned()
        .take(((width * height) / 6).into()) // Fill 1/6 of the terminal area
        .collect_vec();

    let initial_state = State::from(live_cells.as_ref());

    runner::start(&initial_state, &RunnerOptions::default())
}
