use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Block, Borders, Table, Row, Cell};
use tui::layout::Rect;
use crate::process::data::get_processes;
use tui::layout::Constraint;

pub fn render_layout<B: Backend>(f: &mut Frame<B>, layout: &[Rect], scroll_offset: usize) {
    let processes = get_processes();
    let height = layout[0].height as usize - 2;  // Adjust height for header and padding

    // Calculate the visible rows based on the scroll offset
    let visible_processes = &processes[scroll_offset..(scroll_offset + height).min(processes.len())];

    let rows: Vec<Row> = visible_processes
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.2}%", p.cpu_usage)),
                Cell::from(format!("{:.2}%", p.memory_usage)),
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

