use ratatui::{prelude::*, widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState}};
use style::palette::tailwind::{BLUE, GREEN, SLATE};
use std::error;

use crate::model::todo::Todo;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Screen {
    Home
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Home,
            running: true,
            todos: vec![],
            list_state: ListState::default()
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
