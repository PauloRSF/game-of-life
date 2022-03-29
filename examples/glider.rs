use game_of_life::{renderer::Renderer, runner::start, runner::RunnerOptions, state::State};

use terminal_size::{terminal_size, Height, Width};

fn main() {
    let (Width(width), Height(height)) = terminal_size().unwrap();
    let half_width = width as i32 / 2;
    let half_height = height as i32 / 2;

    let mut initial_state = State::new();

    initial_state.put_cell(half_width - 1, half_height + 1);
    initial_state.put_cell(half_width - 1, half_height);
    initial_state.put_cell(half_width - 1, half_height - 1);
    initial_state.put_cell(half_width, half_height + 1);
    initial_state.put_cell(half_width + 1, half_height);

    let renderer = Renderer::new();

    let options = RunnerOptions::defaults();

    start(&initial_state, &renderer, &options);
}
