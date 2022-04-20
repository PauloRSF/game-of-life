use std::io;

use game_of_life::{
    runner::{self, RunnerOptions},
    state::State,
    Cell,
};

fn glider_gun_cells((x, y): (i32, i32)) -> [Cell; 36] {
    [
        (x + 0, y + 3),
        (x + 0, y + 4),
        (x + 1, y + 3),
        (x + 1, y + 4),
        (x + 10, y + 2),
        (x + 10, y + 3),
        (x + 10, y + 4),
        (x + 11, y + 1),
        (x + 11, y + 5),
        (x + 12, y + 0),
        (x + 12, y + 6),
        (x + 13, y + 0),
        (x + 13, y + 6),
        (x + 14, y + 3),
        (x + 15, y + 1),
        (x + 15, y + 5),
        (x + 16, y + 2),
        (x + 16, y + 3),
        (x + 16, y + 4),
        (x + 17, y + 3),
        (x + 20, y + 4),
        (x + 20, y + 5),
        (x + 20, y + 6),
        (x + 21, y + 4),
        (x + 21, y + 5),
        (x + 21, y + 6),
        (x + 22, y + 3),
        (x + 22, y + 7),
        (x + 24, y + 2),
        (x + 24, y + 3),
        (x + 24, y + 7),
        (x + 24, y + 8),
        (x + 34, y + 5),
        (x + 34, y + 6),
        (x + 35, y + 5),
        (x + 35, y + 6),
    ]
}

fn main() -> io::Result<()> {
    let origin = (40, 30);

    let live_cells = glider_gun_cells(origin);

    let initial_state = State::from(live_cells.as_ref());

    runner::start(&initial_state, &RunnerOptions::default())
}

