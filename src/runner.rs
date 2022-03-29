use std::time::Duration;

use crate::{renderer::Renderer, state::State};

pub struct RunnerOptions {
    pub fps: u8,
}

impl RunnerOptions {
    pub fn defaults() -> Self {
        Self { fps: 20 }
    }
}

pub fn start(initial_state: &State, renderer: &Renderer, options: &RunnerOptions) {
    let mut state = initial_state.clone();

    loop {
        renderer.render_frame(&state);
        state = state.next_state();
        // Yeah, i know, this doesn't guarantee consistent fps. It's more of a 'execution speed' kind of deal
        std::thread::sleep(Duration::from_secs_f32(1.0 / options.fps as f32));
    }
}
