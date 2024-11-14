// render.rs

use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Table, Row};
use tui::layout::Rect;
use crate::process::data::get_processes;

pub fn render_layout<B: Backend>(f: &mut Frame<B>, layout: &[Rect]) {
    let processes = get_processes();

    let rows: Vec<Row> = processes
        .iter()
        .map(|p| {
            Row::new(vec![
                p.pid.to_string(),
                p.name.clone(),
                format!("{:.2}%", p.cpu_usage),
                format!("{:.2}%", p.memory_usage),
            ])
        })
        .collect();

    let table = Table::new(rows)
        .header(Row::new(vec!["PID", "Name", "CPU %", "Memory %"]))
        .block(Block::default().borders(Borders::ALL).title("Processes"))
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);

    f.render_widget(table, layout[0]);
}

