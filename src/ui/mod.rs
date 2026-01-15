use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::core::{Task, Priority};

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub mode: Mode,
    pub command_input: String,
    pub show_help: bool,
    pub task_manager: Option<crate::core::TaskManager>,
}

pub enum Mode {
    Normal,
    Command,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            selected: 0,
            mode: Mode::Normal,
            command_input: String::new(),
            show_help: false,
            task_manager: None,
        }
    }

    pub fn next(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = (self.selected + 1) % self.tasks.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.tasks.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.tasks.len() - 1;
            }
        }
    }

    pub fn toggle_done(&mut self) {
        if let Some(task) = self.tasks.get_mut(self.selected) {
            task.done = !task.done;
        }
    }
}

pub fn render(f: &mut Frame, state: &AppState) {
    if state.show_help {
        render_help(f);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header = Paragraph::new("TODO AI - Terminal Task Manager")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = state
        .tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let status = if task.done { "[✓]" } else { "[ ]" };
            let priority = match task.priority {
                Priority::Low => "P1",
                Priority::Medium => "P2",
                Priority::High => "P3",
            };
            let content = format!("{} {} {}", status, priority, task.title);
            
            let style = if i == state.selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if task.done {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));
    f.render_widget(list, chunks[1]);

    let status_text = match state.mode {
        Mode::Normal => {
            "q: quit | j/k: navigate | space: toggle | :: command | ?: help"
        }
        Mode::Command => &format!(":{}", state.command_input),
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn render_help(f: &mut Frame) {
    let help_text = vec![
        Line::from(vec![Span::styled("KEYBOARD SHORTCUTS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  j / ↓     - Move down"),
        Line::from("  k / ↑     - Move up"),
        Line::from("  gg        - Go to top"),
        Line::from("  G         - Go to bottom"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  space     - Toggle task done"),
        Line::from("  d         - Delete task"),
        Line::from("  e         - Edit task"),
        Line::from(""),
        Line::from("Commands:"),
        Line::from("  :add <task>     - Add new task"),
        Line::from("  :done           - Mark as done"),
        Line::from("  :ai <query>     - AI assistant"),
        Line::from("  :sync           - Sync tasks"),
        Line::from(""),
        Line::from("Other:"),
        Line::from("  ?         - Toggle this help"),
        Line::from("  q         - Quit"),
        Line::from(""),
        Line::from(vec![Span::styled("Press ? to close", Style::default().fg(Color::Yellow))]),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .alignment(Alignment::Left);
    
    let area = f.size();
    f.render_widget(help, area);
}