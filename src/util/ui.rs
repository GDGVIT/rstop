use crate::util::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    symbols,
    widgets::{Block, Borders, Chart, Dataset, Row, Table},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .vertical_margin(2)
        .split(f.size());

    draw_first_row(f, app, chunks[0]);
    draw_second_row(f, app, chunks[1]);
    draw_third_row(f, app, chunks[2]);
}

fn draw_first_row<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    //let block = Block::default().borders(Borders::ALL).title(" CPU Usage ");
    let datasets = vec![Dataset::default()
        .name(" CPU0 ")
        .marker(symbols::Marker::Dot)
        .data(&app.cpu_usage_points)];

    let chart =
        Chart::new(datasets).block(Block::default().title(" Disk Usage ").borders(Borders::ALL));
    f.render_widget(chart, area);
}

fn draw_second_row<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
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

    let rows = app.disk_usage.iter().map(|x| Row::new(x.clone()));
    let table = Table::new(rows)
        .header(Row::new(vec!["Name", "Mount", "Free"]))
        .block(
            Block::default()
                .title(" Memory Usage ")
                .borders(Borders::ALL),
        )
        .widths(&[
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ]);
    f.render_widget(table, chunks[0]);

    let rows = app.temps.iter().map(|x| Row::new(x.clone()));
    let table = Table::new(rows)
        .block(
            Block::default()
                .title(" Temperatures ")
                .borders(Borders::ALL),
        )
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);
    f.render_widget(table, chunks[1]);

    let block = Block::default().borders(Borders::ALL).title(" Disk Usage ");
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
