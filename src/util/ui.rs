use crate::logger::Logger;
use crate::util::{app::SortBy, App};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Row, Sparkline, Table, TableState},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, logger: &mut Logger) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .vertical_margin(2)
        .split(f.size());

    draw_first_row(f, app, chunks[0], logger);
    draw_second_row(f, app, chunks[1], logger);
    draw_third_row(f, app, chunks[2]);
}

fn map_color_to_index(i: usize) -> Color {
    match i {
        0 => Color::Cyan,
        1 => Color::Yellow,
        2 => Color::Red,
        3 => Color::Blue,
        4 => Color::Green,
        5 => Color::Magenta,
        _ => Color::White,
    }
}

fn draw_first_row<B>(f: &mut Frame<B>, app: &mut App, area: Rect, _logger: &mut Logger)
where
    B: Backend,
{
    let mut datasets = vec![];
    for (i, ele) in app.cpu_usage_points.iter().enumerate() {
        datasets.push(
            Dataset::default()
                .name(format!(" CPU{} ", i))
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(map_color_to_index(i)))
                .data(&ele),
        );
    }

    //let chart_legend_constraints = (Constraint::Ratio(1, 3), Constraint::Ratio(1, 4));

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    " CPU Usage ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        //.hidden_legend_constraints(chart_legend_constraints)
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![])
                .bounds([0.0, 20.0]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![])
                .bounds([-20.0, 20.0]),
        );
    f.render_widget(chart, area);
}

fn draw_second_row<B>(f: &mut Frame<B>, app: &mut App, area: Rect, logger: &mut Logger)
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

    draw_disk_usage(f, app, chunks_horiz[1], logger);
}

fn draw_disk_usage<B>(f: &mut Frame<B>, app: &mut App, area: Rect, logger: &mut Logger)
where
    B: Backend,
{
    let datasets = vec![
        Dataset::default()
            .name(format!(" Memory "))
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Red))
            .data(app.memory.memory_queue.vec()),
        Dataset::default()
            .name(format!(" Swap "))
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Yellow))
            .data(app.memory.swap_queue.vec()),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    " Disk Usage ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        //.hidden_legend_constraints(chart_legend_constraints)
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![])
                .bounds([0.0, 20.0]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(vec![])
                .bounds([-20.0, 20.0]),
        );

    if let Ok(_) = logger.add_log(format!(
        "Memory Chart Data Received: {:?} \n Swap Chart Data Received: {:?}",
        app.memory.memory_queue, app.memory.swap_queue
    )) {}

    f.render_widget(chart, area);
}

fn draw_network_section<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Network Usage ");
    f.render_widget(block, area);

    let sparkline = Sparkline::default()
        .block(Block::default().title("Total RX"))
        .style(Style::default().fg(Color::Cyan))
        .data(app.network.rx_queue.vec());
    f.render_widget(sparkline, chunks[0]);

    let sparkline = Sparkline::default()
        .block(Block::default().title("Total TX"))
        .style(Style::default().fg(Color::Blue))
        .data(app.network.tx_queue.vec());
    f.render_widget(sparkline, chunks[1]);
}

fn draw_process_section<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let rows = app.process.process_list.iter().map(|s| {
        Row::new(vec![
            s.0.to_string(),
            s.1.to_string(),
            s.2.to_string(),
            s.3.to_string(),
        ])
        .style(Style::default().fg(Color::Cyan))
    });
    let table = Table::new(rows)
        .header(
            match app.process.sort_by {
                SortBy::CPU => Row::new(vec!["Pid", "Process", "CPU^", "Mem"]),
                SortBy::MEMORY => Row::new(vec!["Pid", "Process", "CPU", "Mem^"]),
            }
            .style(Style::default().fg(Color::Yellow))
            .bottom_margin(1),
        )
        .block(Block::default().title(" Processes ").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::LightRed))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(70),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
        ]);

    let mut state = TableState::default();
    state.select(Some(app.process.active_index));

    f.render_stateful_widget(table, area, &mut state);
}

fn draw_third_row<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .direction(Direction::Horizontal)
        .split(area);

    draw_network_section(f, app, chunks[0]);
    draw_process_section(f, app, chunks[1]);
}
