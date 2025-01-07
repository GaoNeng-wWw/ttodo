use crate::{
    app::{App, Focus},
    model::todo::{Todo, TodoStatus},
};
use crossterm::event::{Event, KeyCode, KeyEvent};
use layout::Flex;
use ratatui::{
    prelude::*,
    widgets::{Block, Clear, List, ListItem, Paragraph, Row, Table, TableState},
};
use std::vec;
use style::palette::tailwind::{SLATE, ZINC};
use tui_input::backend::crossterm::EventHandler;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }
    pub fn handle_key(&mut self, key_event: KeyEvent, app: &mut App) {
        if app.show_popup {
            match key_event.code {
                KeyCode::Enter => {
                    if app.home_add_todo_focus == Focus::None {
                        let todo_name = app.home_add_todo_todo_name_input.value();
                        let todo = self.create_todo(todo_name);
                        app.todos.push(todo);
                        app.home_add_todo_todo_summary_input.reset();
                        app.home_add_todo_todo_name_input.reset();
                    }
                    let evt = &Event::Key(key_event);
                    let _ = match app.home_add_todo_focus {
                        Focus::None => (),
                        Focus::Name => {
                            app.home_add_todo_todo_name_input.handle_event(evt);
                            ()
                        }
                    };
                }
                KeyCode::Up => match app.home_add_todo_focus {
                    Focus::None => app.home_add_todo_focus = Focus::Name,
                    Focus::Name => app.home_add_todo_focus = Focus::None,
                },
                KeyCode::Down => match app.home_add_todo_focus {
                    Focus::None => app.home_add_todo_focus = Focus::Name,
                    Focus::Name => app.home_add_todo_focus = Focus::None,
                },
                KeyCode::Char('Q') | KeyCode::Esc | KeyCode::Char('q') => {
                    app.show_popup = false;
                }
                _ => {
                    let evt = &Event::Key(key_event);
                    let _ = match app.home_add_todo_focus {
                        Focus::None => (),
                        Focus::Name => {
                            app.home_add_todo_todo_name_input.handle_event(evt);
                            ()
                        }
                    };
                }
            }
            return;
        }
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => self.select_next(app),
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => self.select_prev(app),
            KeyCode::Char('G') => self.select_last(app),
            KeyCode::Char('g') => self.select_first(app),
            KeyCode::Char('x') => self.set_status(app, TodoStatus::DONE),
            KeyCode::Char('e') => {
                app.show_popup = true;
            }
            _ => {}
        }
    }
    pub fn create_todo(&self, name: &str) -> Todo {
        Todo {
            name: String::from(name),
            status: TodoStatus::PROGESS,
        }
    }
    pub fn set_status(&self, app: &mut App, status: TodoStatus) {
        let idx = app.list_state.selected();
        if idx.is_none() {
            return;
        }
        let idx = idx.unwrap();
        app.todos[idx].status = status;
    }
    pub fn select_first(&self, app: &mut App) {
        app.list_state.select_first();
    }
    pub fn select_last(&self, app: &mut App) {
        app.list_state.select_last();
    }
    pub fn select_prev(&self, app: &mut App) {
        app.list_state.select_previous();
    }
    pub fn select_next(&self, app: &mut App) {
        app.list_state.select_next();
    }
    pub fn render(&self, app: &mut App, frame: &mut Frame) {
        let area = frame.area();
        let buf = frame.buffer_mut();
        let [top, bottom] = Layout::vertical([Constraint::Fill(1), Constraint::Max(1)]).areas(area);
        let [left, right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(top);
        self.render_todo_list_area(app, left, buf);
        self.render_selected_todo(app, right, buf);
        if app.show_popup {
            let popup = self.render_popup(area, 50, 25);
            let block = Block::bordered().title("Add Todo Item");

            let width = popup.width;
            let scroll = app
                .home_add_todo_todo_name_input
                .visual_scroll(width as usize);
            let active_style = Style::default().fg(Color::Yellow);
            let todo_name_widget = Paragraph::new(app.home_add_todo_todo_name_input.value())
                .style(match app.home_add_todo_focus {
                    Focus::Name => active_style,
                    _ => Style::default(),
                })
                .scroll((0, scroll as u16))
                .block(
                    Block::bordered()
                        .border_set(symbols::border::ROUNDED)
                        .title("Todo Name"),
                );

            frame.render_widget(Clear, popup);
            frame.render_widget(block, popup);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Fill(1)])
                .split(popup);

            frame.render_widget(todo_name_widget, chunks[0]);
        }
    }
    fn render_popup(&self, area: Rect, width: u16, height: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(height)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(width)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [popup] = horizontal.areas(area);
        return popup;
    }
    fn render_todo_list(&self, app: &mut App, area: Rect, buf: &mut Buffer, block: Block) {
        let items: Vec<ListItem> = app
            .todos
            .iter()
            .map(|item| ListItem::new(item.name.clone()))
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE);
        StatefulWidget::render(list, area, buf, &mut app.list_state);
    }
    fn render_todo_list_area(&self, app: &mut App, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Todo List").bold();
        let bottom_title = Line::from(vec![
            " Quit ".into(),
            "<q>".blue().into(),
            " Create ".into(),
            "<e>".blue().into(),
            " Done ".into(),
            "<x> ".blue().into(),
        ]);
        let block = Block::bordered()
            .title(title)
            .border_set(symbols::border::ROUNDED)
            .title_bottom(bottom_title.centered());
        self.render_todo_list(app, area, buf, block);
    }
    fn get_current_item(&self, app: &mut App) -> Option<Todo> {
        let idx = app.list_state.selected();
        if idx.is_none() {
            return None;
        }
        let idx = idx.unwrap();
        return Some(app.todos[idx].clone());
    }
    fn render_selected_todo(&self, app: &mut App, area: Rect, buf: &mut Buffer) {
        let info = Block::bordered().border_set(symbols::border::ROUNDED);
        let todo = self.get_current_item(app);
        if todo.is_none() {
            return;
        }
        let todo: Todo = todo.unwrap();
        let progress_string = match todo.status {
            TodoStatus::DONE => "Done",
            TodoStatus::PROGESS => "Progress",
        };
        let header = ["Name:", "Summary:", "Progress:"];
        let mut width = 0;
        for head in header {
            width = std::cmp::max(width, head.len())
        }
        let widths = [
            Constraint::Length(width.try_into().unwrap()),
            Constraint::Fill(1),
        ];
        let rows = [
            Row::new(vec!["Name:", &todo.name]),
            Row::new(vec!["Progress:", progress_string]),
        ];
        let table = Table::new(rows, widths).block(info);
        let mut table_state = TableState::default();
        StatefulWidget::render(table, area, buf, &mut table_state);
    }
}
