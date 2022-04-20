use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::DOT,
    text::{Span, Spans},
    widgets::{canvas::Canvas, Block},
    Frame,
};

use crate::{runner, state::State};

fn separator<'a>() -> Span<'a> {
    Span::raw(" | ")
}

fn commands<'a>() -> Vec<Span<'a>> {
    vec![
        Span::raw("Press "),
        Span::styled(
            runner::QUIT_CHAR.to_string(),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" to quit"),
    ]
}

fn runner_status<'a>() -> Span<'a> {
    Span::styled(" RUNNING", Style::default().add_modifier(Modifier::BOLD))
}

fn bottom_bar<'a>() -> Spans<'a> {
    let mut spans = vec![runner_status(), separator()];

    spans.append(&mut commands());

    Spans::from(spans)
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, state: &State) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());

    let board = Canvas::default()
        .block(Block::default())
        .paint(|ctx| {
            state.live_cells.iter().for_each(|&(x, y)| {
                ctx.print(x.into(), y.into(), DOT);
            });
            ctx.print(0.0, 0.0, bottom_bar())
        })
        .x_bounds([0.0, frame.size().width.into()])
        .y_bounds([0.0, frame.size().height.into()]);

    frame.render_widget(board, layout[0]);
}
