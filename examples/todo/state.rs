/// Todo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub title: String,
    pub desc: Option<String>,
    pub done: bool
}
impl Todo {
    pub fn new<T, D>(title: T, desc: Option<D>, done: bool) -> Self
    where T: ToString,
          D: ToString
    {
        Self {
            title: title.to_string(),
            desc: desc.and_then(|d| Some(d.to_string())),
            done
        }
    }

    //

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
}

/// Page
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    #[default]
    Todos,
    NewTodo
}

/// State
#[derive(Debug)]
pub struct State {
    pub page: Page,
    pub todos: Vec<Todo>,
    pub cur_todo: usize,
}
impl State {
    pub fn new() -> Self {
        Self {
            page: Page::default(),
            todos: vec![
                Todo::new::<_, String>("Im a todo without a description!", None, false),
                Todo::new(
                    "I have a description",
                    Some("Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain"),
                    false
                ),
                Todo::new("This task is done", Some("that's all..."), true),
                Todo::new::<_, String>("Im a todo without a description!", None, false),
                Todo::new("This task is done", Some("that's all..."), true),
                Todo::new(
                    "I have a description",
                    Some("Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain"),
                    false
                ),
                Todo::new::<_, String>("Im a todo without a description!", None, false),
                Todo::new("This task is done", Some("that's all..."), true),
                Todo::new(
                    "I have a description",
                    Some("Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain"),
                    false
                ),
                Todo::new("This task is done", Some("that's all..."), true),
                Todo::new::<_, String>("Im a todo without a description!", None, false),
                Todo::new(
                    "I have a description",
                    Some("Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain"),
                    false
                ),
                Todo::new(
                    "I have a description",
                    Some("Nor again is there anyone who loves or pursues or desires to obtain pain of itself, because it is pain"),
                    false
                ),
            ],
            cur_todo: 0
        }
    }

    //

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.insert(0, todo);
    }
    pub fn remove_todo(&mut self, index: usize) -> bool {
        if index >= self.todos.len() {
            false
        } else {
            self.todos.remove(index);
            self.clamp_cur_todo();
            true
        }
    }

    pub fn select_next_todo(&mut self, jump: usize) {
        self.cur_todo += jump;
        self.clamp_cur_todo();
    }
    pub fn select_prev_todo(&mut self, jump: usize) {
        self.cur_todo = self.cur_todo.saturating_sub(jump);
    }
    pub fn clamp_cur_todo(&mut self) {
        self.cur_todo = self.cur_todo.min(self.todos.len().saturating_sub(1));
    }

    pub fn get_cur_todo_mut(&mut self) -> &mut Todo {
        self.get_todo_mut(self.cur_todo).unwrap()
    }

    pub fn get_todo_mut(&mut self, index: usize) -> Option<&mut Todo> {
        self.todos.get_mut(index)
    }
}
