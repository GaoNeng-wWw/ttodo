use crate::{
    app::{App, AppResult},
    views::home::Home,
};
use crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let mut home = Home::new();
    match app.screen {
        crate::app::Screen::Home => home.handle_key(key_event, app),
    }
    Ok(())
}
