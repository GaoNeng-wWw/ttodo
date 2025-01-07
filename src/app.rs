use crate::model::todo::Todo;
use ratatui::widgets::ListState;
use std::error;
use tui_input::Input;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub enum Screen {
    Home,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Focus {
    None,
    Name,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    /// Is the application running?
    pub running: bool,
    /// todos
    pub todos: Vec<Todo>,
    /// List State
    pub list_state: ListState,
    /// Input State
    /// Used to control Todo Add
    pub input_state: InputMode,
    pub todo_name: String,
    pub input: Input,
    pub show_popup: bool,
    pub home_add_todo_focus: Focus,
    pub home_add_todo_todo_name_input: Input,
    pub home_add_todo_todo_summary_input: Input,
    pub home_add_todo_todo_name: String,
    pub home_add_todo_todo_summary: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            running: true,
            todos: vec![],
            list_state: ListState::default(),
            input_state: InputMode::Normal,
            todo_name: String::from(""),
            input: Input::default(),
            show_popup: false,
            home_add_todo_focus: Focus::Name,
            home_add_todo_todo_name_input: Input::default(),
            home_add_todo_todo_summary_input: Input::default(),
            home_add_todo_todo_name: String::from(""),
            home_add_todo_todo_summary: String::from(""),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
