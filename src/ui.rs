use lazy_static::lazy_static;
use ratatui::prelude::*;

use crate::{
    app::{App, Screen},
    views::home::Home,
};

lazy_static! {
    static ref home: Home = Home::new();
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    match app.screen {
        Screen::Home => home.render(app, frame),
    }
}
