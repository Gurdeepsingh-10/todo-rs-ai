use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.size());

    let block = Block::default()
        .title("TODO TUI - Press 'q' to quit")
        .borders(Borders::ALL);
    
    let paragraph = Paragraph::new("Task list will appear here").block(block);
    f.render_widget(paragraph, chunks[0]);

    let status = Paragraph::new("Status bar")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}