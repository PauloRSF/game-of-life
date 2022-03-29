use game_of_life::{renderer::Renderer, runner::start, runner::RunnerOptions, state::State};

use terminal_size::{terminal_size, Height, Width};

fn main() {
    let (Width(width), Height(height)) = terminal_size().unwrap();

    let live_cell_count = (width * height) / 6;

    let initial_state = State::new_randomized(live_cell_count.into());

    let renderer = Renderer::new();

    let options = RunnerOptions::defaults();

    start(&initial_state, &renderer, &options);
}
