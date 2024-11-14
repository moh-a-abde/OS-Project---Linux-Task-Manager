use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Table, Row, Cell, Paragraph, Wrap};
use tui::layout::{Rect, Constraint, Alignment};
use tui::style::{Style, Modifier, Color};
use crate::process::data::ProcessUsage;

pub fn render_layout<B: Backend>(
    f: &mut Frame<B>,
    layout: &[Rect],
    scroll_offset: usize,
    input: &str,
    command_output: &str,
    processes: &[ProcessUsage],
) {
    let height = layout[0].height as usize - 2;

    let visible_processes = &processes[scroll_offset..(scroll_offset + height).min(processes.len())];

    let header_cells = ["PID", "Name", "CPU %", "Memory %"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));

    let rows: Vec<Row> = visible_processes
        .iter()
        .map(|p| {
            let cells = vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}%", p.cpu_usage)),
                Cell::from(format!("{:.2}%", p.memory_usage)),
            ];
            Row::new(cells)
        })
        .collect();

    let table = Table::new(rows)
        .header(Row::new(header_cells))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Processes")
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::White)),
        )
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

    f.render_widget(table, layout[0]);

    let input_text = Paragraph::new(format!("Input: {}", input))
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command Input")
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(input_text, layout[1]);

    let output_text = Paragraph::new(command_output)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command Output")
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(output_text, layout[2]);
}

