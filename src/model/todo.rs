#[derive(Debug, Clone)]
pub enum TodoStatus{
    DONE,
    PROGESS
}
#[derive(Debug,Clone)]
pub struct Todo {
    pub name: String,
    pub summary: String,
    pub status: TodoStatus
}

impl Todo {
    pub fn new(
        name:String,
        summary:String,
        status:TodoStatus
    ) -> Self {
        Self { name, summary, status }
    }
}