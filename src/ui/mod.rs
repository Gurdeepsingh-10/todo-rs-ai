use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::core::{Task, Priority};

use crate::config::{Config, CommandHistory};

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub mode: Mode,
    pub command_input: String,
    pub show_help: bool,
    pub task_manager: Option<crate::core::TaskManager>,
    pub editing_task: Option<String>,
    pub current_user: Option<crate::sync::User>,
    pub sync_manager: Option<crate::sync::SyncManager>,
    pub config: Config,                    // Add this
    pub command_history: CommandHistory,   // Add this
}

pub enum Mode {
    Normal,
    Command,
    Edit,
    Login,      // Add this
    Register,
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
            editing_task: None,
            current_user: None,
            sync_manager: None,
            config: Config::load(),
            command_history: CommandHistory::new(),
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

    
}

pub fn render(f: &mut Frame, state: &AppState) {
    if state.show_help {
        render_help(f);
        return;
    }

    // Show login prompt if not logged in
    if state.current_user.is_none() && matches!(state.mode, Mode::Login | Mode::Register) {
        render_auth(f, state);
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
        Mode::Edit => &format!("Edit: {}", state.command_input),
        Mode::Login => "Login mode",
        Mode::Register => "Register mode",
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
        Line::from(vec![Span::styled("Navigation:", Style::default().fg(Color::Yellow))]),
        Line::from("  j / ↓         - Move down"),
        Line::from("  k / ↑         - Move up"),
        Line::from("  gg            - Go to top"),
        Line::from("  G             - Go to bottom"),
        Line::from(""),
        Line::from(vec![Span::styled("Actions:", Style::default().fg(Color::Yellow))]),
        Line::from("  space         - Toggle task done/undone"),
        Line::from("  d             - Delete selected task"),
        Line::from("  e             - Edit selected task"),
        Line::from("  1/2/3         - Set priority (1=Low, 2=Med, 3=High)"),
        Line::from("  ?             - Toggle this help screen"),
        Line::from("  q             - Quit application"),
        Line::from(""),
        Line::from(vec![Span::styled("Command Mode (press :)", Style::default().fg(Color::Yellow))]),
        Line::from("  :add <task>              - Add new task"),
        Line::from("                             Example: :add Buy groceries"),
        Line::from(""),
        Line::from("  :done                    - Mark selected task as done"),
        Line::from(""),
        Line::from("  :sync              - Sync tasks"),
        Line::from("  :quit or :q              - Quit application"),
        Line::from(""),
        Line::from(vec![Span::styled("Edit/Command Mode Controls:", Style::default().fg(Color::Yellow))]),
        Line::from("  Esc           - Cancel and exit mode"),
        Line::from("  Enter         - Confirm changes"),
        Line::from("  Backspace     - Delete character"),
        Line::from(""),
        Line::from(vec![Span::styled("Press ? to close this help", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))]),
        Line::from(vec![Span::styled("Configuration:", Style::default().fg(Color::Yellow))]),
        Line::from("  :config                  - Show config location"),
        Line::from("  :config <action> <key>   - Change keybinding"),
        Line::from("                             Example: :config move_down h"),
        Line::from(""),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help & Commands"))
        .alignment(Alignment::Left);
    
    let area = f.size();
    f.render_widget(help, area);
}

fn render_auth(f: &mut Frame, state: &AppState) {
    let (title, instruction) = match state.mode {
        Mode::Login => ("Login", "Format: username password | Press : to register"),
        Mode::Register => ("Register", "Format: username password email | Press Esc to go back"),
        _ => ("Auth", ""),
    };

    let text = vec![
        Line::from(vec![Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from(instruction),
        Line::from(""),
        Line::from(format!("> {}", state.command_input)),
    ];
    
    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Authentication"))
        .alignment(Alignment::Left);
    
    f.render_widget(para, f.size());
}