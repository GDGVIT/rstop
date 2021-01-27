use crate::util::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(f.size());

    draw_first_row(f, app, chunks[0]);
    draw_second_row(f, app, chunks[1]);
    draw_third_row(f, app, chunks[2]);
}

fn draw_first_row<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL).title(" CPU Usage ");
    f.render_widget(block, area);
}

fn draw_second_row<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks_horiz = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(Direction::Horizontal)
        .split(area);

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(Direction::Vertical)
        .split(chunks_horiz[0]);

    let block = Block::default().borders(Borders::ALL).title(" Disk Usage ");
    f.render_widget(block, chunks[0]);

    let block = Block::default().borders(Borders::ALL).title(" CPU Usage ");
    f.render_widget(block, chunks[1]);

    let block = Block::default().borders(Borders::ALL).title(" CPU Usage ");
    f.render_widget(block, chunks_horiz[1]);
}

fn draw_third_row<B>(f: &mut Frame<B>, _app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(Direction::Horizontal)
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Network Usage ");
    f.render_widget(block, chunks[0]);

    let block = Block::default().borders(Borders::ALL).title(" Processes ");
    f.render_widget(block, chunks[1]);
}
