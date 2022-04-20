use std::io;

use game_of_life::{
    runner::{self, RunnerOptions},
    state::State,
};

fn main() -> io::Result<()> {
    let live_cells = [(2, 2), (2, 1), (2, 0), (1, 2), (0, 1)].as_ref();

    let initial_state = State::from(live_cells);

    runner::start(&initial_state, &RunnerOptions::default())
}
