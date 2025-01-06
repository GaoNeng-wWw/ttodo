use std::vec;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::{Block, List, ListItem, Paragraph, Row, Table, TableState}};
use style::palette::tailwind::SLATE;
use tui_input::{backend::crossterm::EventHandler, Input};
use crate::{app::App, model::todo::{Todo, TodoStatus}};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug)]
enum InputMode {
    Nomral,
    Editor
}

pub struct Home {
    input:Input,
    todo_name:String,
    input_mode:InputMode
}

impl Home {
    pub fn new() -> Self {
        Self {
            input: Input::default(),
            todo_name: String::from(""),
            input_mode: InputMode::Nomral
        }
    }
    pub fn handle_key(&mut self, key_event: KeyEvent, app: &mut App){
        match self.input_mode {
            InputMode::Nomral => {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => self.select_next(app),
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => self.select_prev(app),
                    KeyCode::Char('G') => self.select_last(app),
                    KeyCode::Char('g') => self.select_first(app),
                    KeyCode::Char('x') => self.set_status(app, TodoStatus::DONE),
                    KeyCode::Char('e') => {
                        self.input_mode = InputMode::Editor;
                        println!("{:?}", self.input_mode);
                    },
                    _ => {}
                }
            },
            InputMode::Editor =>{
                println!("{:?}", self.input_mode);
                match key_event.code {
                    KeyCode::Enter=>{
                        self.todo_name = self.input.value().to_string();
                        self.input.reset();
                    }
                    KeyCode::Esc => {
                        self.input_mode = InputMode::Nomral;
                    }
                    _ => {
                        self.input.handle_event(&Event::Key(key_event));
                    }
                }
            }
        }
    }
    pub fn set_status(&self, app: &mut App, status: TodoStatus){
        let idx = app.list_state.selected();
        if idx.is_none() {
            return;
        }
        let idx = idx.unwrap();
        app.todos[idx].status = status;

    }
    pub fn select_first(&self, app: &mut App){
        app.list_state.select_first();
    }
    pub fn select_last(&self, app: &mut App){
        app.list_state.select_last();
    }
    pub fn select_prev(&self, app: &mut App){
        app.list_state.select_previous();
    }
    pub fn select_next(&self, app: &mut App){
        app.list_state.select_next();
    }
    pub fn render(
        &self,
        app: &mut App,
        frame: &mut Frame
    ){
        let area = frame.area();
        let buf = frame.buffer_mut();
        let [top, bottom] = Layout::vertical(
            [
                Constraint::Max(3),
                Constraint::Min(5)
            ]
        ).areas(area);

        let [left, right] = Layout::horizontal(
            [
                Constraint::Fill(1),
                Constraint::Fill(1)
            ]
        )
        .areas(bottom);
        self.render_quick_add(app, top, buf);
        self.render_todo_list_area(app,left, buf);
        self.render_selected_todo(app, right, buf);
    }
    fn render_quick_add(&self, app: &mut App, area:Rect,buf: &mut Buffer){
        let width = area.width;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(
            self.input.value()
        )
        .style(
            match self.input_mode {
                InputMode::Nomral => Style::default(),
                InputMode::Editor => Style::default().fg(Color::Yellow)
            }
        )
        .scroll((0,scroll as u16))
        .block(Block::bordered().border_set(symbols::border::ROUNDED).title("Input"))
        .render(area, buf);
    }
    fn render_todo_list(
        &self,
        app: &mut App,
        area:Rect,
        buf: &mut Buffer,
        block: Block
    ){
        let total = 100;
        let mut list_items =vec![];
        for i in 0..total {
            let todo = Todo::new(format!("{}",i), format!("{} summary", i), crate::model::todo::TodoStatus::PROGESS);
            app.todos.push(todo.clone());
            list_items.push(
                ListItem::new(todo.name)
            );
        }
        let list= List::new(list_items)
        .block(block)
        .highlight_style(SELECTED_STYLE);
        StatefulWidget::render(list, area, buf, &mut app.list_state);
    }
    fn render_todo_list_area(
        &self,
        app: &mut App,
        area:Rect,
        buf: &mut Buffer
    ){
        let title = Line::from("Todo List").bold();
        let block = Block::bordered()
        .title(title)
        .border_set(symbols::border::ROUNDED);
        self.render_todo_list(app, area, buf, block);
    }
    fn get_current_item(&self, app: &mut App) -> Option<Todo>{
        let idx = app.list_state.selected();
        if idx.is_none() {
            return None;
        }
        let idx = idx.unwrap();
        return Some(app.todos[idx].clone());
    }
    fn render_selected_todo(
        &self,
        app: &mut App,
        area:Rect,
        buf: &mut Buffer,
    ){
        let info = Block::bordered().border_set(symbols::border::ROUNDED);
        let todo = self.get_current_item(app);
        if todo.is_none(){
            return;
        }
        let todo = todo.unwrap();
        let progress_string = match todo.status {
            TodoStatus::DONE => "Done",
            TodoStatus::PROGESS => "Progress"
        };
        let header = ["Name:","Summary:","Progress:"];
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
            Row::new(vec!["Summary:", &todo.summary]),
            Row::new(vec!["Progress:", progress_string]),
        ];
        let table = Table::new(rows, widths)
        .block(info);
        let mut table_state = TableState::default();
        StatefulWidget::render(table, area, buf, &mut table_state);
    }
}