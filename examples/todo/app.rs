use std::ops::Not;

use tuich::{buffer::Buffer, event::{Event, Key, KeyCode}, layout::{Align, Clip, Rect}, state::PromptState, style::{BorderKind, Color}, text::Text, widget::{Block, Draw, Prompt, RefDraw}};

use crate::{state::{Page, State, Todo}, widget::todo::TodoList, Msg};

/// Main message
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMsg {
    #[default]
    None
}

/// App
#[derive(Debug)]
pub struct App {
    pub state: State,

    pub todo_title_prompt: PromptState,
    pub todo_desc_prompt: PromptState,
    pub is_desc_focused: bool,
}
impl App {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            todo_title_prompt: PromptState::default(),
            todo_desc_prompt: PromptState::default(),
            is_desc_focused: false
        }
    }

    //

    pub fn handle_events(&mut self, event: Event) -> Msg {
        match event {
            Event::Key(key, _)  => self.handle_keys(key),
            Event::Resize(w, h) => Msg::Resize(w, h),
            _                   => Msg::None
        }
    }
    fn handle_keys(&mut self, key: Key) -> Msg {
        match self.state.page {
            Page::Todos => self.todos_handle_key(key),
            Page::NewTodo => self.new_todo_handle_key(key)
        }
    }

    fn todos_handle_key(&mut self, key: Key) -> Msg {
        match key.code() {
            KeyCode::Char('q') => Msg::Quit,
            KeyCode::Char('j') => self.select_next_todo(),
            KeyCode::Char('k') => self.select_prev_todo(),

            KeyCode::Char('d') |
            KeyCode::Backspace => self.remove_cur_todo(),

            KeyCode::Char('l') |
            KeyCode::Char(' ') |
            KeyCode::Enter     => self.toggle_cur_todo(),

            KeyCode::Char('n') => self.goto_page(Page::NewTodo),

            _ => Msg::None
        }
    }
    fn new_todo_handle_key(&mut self, key: Key) -> Msg {
        match key {
            Key(_, KeyCode::Esc) => self.goto_page(Page::Todos),
            Key(_, KeyCode::Tab) => {
                self.is_desc_focused = !self.is_desc_focused;
                Msg::Draw
            },
            Key(_, KeyCode::Enter) => self.add_todo(),

            key => {
                if self.is_desc_focused {
                    self.todo_desc_prompt.handle_keys(key).into()
                } else {
                    self.todo_title_prompt.handle_keys(key).into()
                }
            }
        }
    }

    fn goto_page(&mut self, page: Page) -> Msg {
        self.state.page = page;
        Msg::Draw
    }

    fn add_todo(&mut self) -> Msg {
        let title = self.todo_title_prompt.value();
        let desc = self.todo_desc_prompt.value();

        if title.is_empty() {
            return Msg::None;
        }

        self.state.add_todo(Todo::new(
            title,
            desc.is_empty()
                .not()
                .then_some(desc),
            false
        ));

        self.is_desc_focused = false;
        self.todo_title_prompt.clear();
        self.todo_desc_prompt.clear();

        self.goto_page(Page::Todos)
    }

    fn toggle_cur_todo(&mut self) -> Msg {
        self.state.get_cur_todo_mut().toggle();
        Msg::Draw
    }
    fn remove_cur_todo(&mut self) -> Msg {
        self.state.remove_todo(self.state.cur_todo);
        Msg::Draw
    }
    fn select_next_todo(&mut self) -> Msg {
        self.state.select_next_todo(1);
        Msg::Draw
    }
    fn select_prev_todo(&mut self) -> Msg {
        self.state.select_prev_todo(1);
        Msg::Draw
    }
}

impl RefDraw for App {
    fn draw(&self, buf: &mut Buffer, rect: Rect) -> Rect {
        let rect = Rect::sized(60, 24)
            .min_size(rect.size())
            .align_center(rect);

        match self.state.page {
            Page::Todos => draw_todos_page(self, buf, rect),
            Page::NewTodo => draw_new_todo_page(self, buf, rect)
        }

        rect
    }
}

fn draw_todos_page(app: &App, buf: &mut Buffer, rect: Rect) {
    let borders_rect = Block::default()
        .title(
            Text::new(" TODOS ", Color::Green)
                .align(Align::Center)
        )
        .footer(
            Text::new(" [j/k] - select  [n] - new  [d] - delete  [l] - toggle ", Color::Gray)
                .clip(Clip::Ellipsis)
        )
        .style(Color::LightBlack)
        .draw(buf, rect)
        .margin(1);

    TodoList(&app.state.todos, app.state.cur_todo)
        .draw(buf, borders_rect);
}

fn draw_new_todo_page(app: &App, buf: &mut Buffer, rect: Rect) {
    let rect = rect
        .with_height(11)
        .align_center(rect);

    let borders_rect = Block::default()
        .title(
            Text::new(" NEW TODO ", Color::Green)
                .align(Align::Center)
        )
        .footer(
            Text::new(" [esc] - cancel  [enter] - add  [tab] - focus ", Color::Gray)
                .clip(Clip::Ellipsis)
        )
        .style(Color::LightBlack)
        .draw(buf, rect)
        .margin(1);

    //let rects = Stack::new([
    //    Length::value(1),
    //    Length::value(3),
    //
    //    Length::value(1),
    //    Length::value(3),
    //].as_ref())
    //    .calc(borders_rect);

    Text::new("Todo title:", Color::Gray)
        .draw(buf, borders_rect.with_height(1));
    Prompt::new(&app.todo_title_prompt)
        .border_kind(BorderKind::Single)
        .border_style(Color::Gray)
        .focused(!app.is_desc_focused)
        .draw(buf, borders_rect.add_y(1).with_height(3));

    Text::new("Todo description (optional):", Color::Gray)
        .draw(buf, borders_rect.add_y(4).with_height(1));
    Prompt::new(&app.todo_desc_prompt)
        .border_kind(BorderKind::Single)
        .border_style(Color::Gray)
        .focused(app.is_desc_focused)
        .draw(buf, borders_rect.add_y(5).with_height(3));
}
