use tuich::{buffer::Buffer, layout::{Align, Rect, Wrap}, style::{BorderKind, Color, Style, Stylized}, text::Text, widget::{Clear, Draw, Line, List}};

use crate::state::Todo;

// Consts
pub const DONE_COLOR: Color = Color::Green;

/// Todo widget
pub struct TodoWidget<'a>(pub &'a Todo, pub bool);
impl<'a> Draw for TodoWidget<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let todo = self.0;
        let is_cur = self.1;

        let title = todo.title.clone();

        let checkbox_style: Style =
            if todo.done { DONE_COLOR.into() }
            else { Color::Gray.into() };
        let title_style: Style =
            if todo.done { DONE_COLOR.into() }
            else { Color::Reset.into() };
        let desc_style = Style::default()
            .fg(Color::Gray)
            .italic(true);
        let clear_style: Style =
            if todo.done { (Color::Black, DONE_COLOR).into() }
            else { (Color::Black, Color::LightGray).into() };

        let checkbox =
            if todo.done { "[x]" }
            else { "[ ]" };
        let checkbox_rect = Text::new(checkbox, checkbox_style)
            .draw(buf, rect);

        let text_rect = if let Some(desc) = &todo.desc {
            List::col([
                Text::new(title, title_style)
                    .wrap(Wrap::BreakWords),
                Text::new(desc, desc_style)
                    .wrap(Wrap::BreakWords)
            ])
        } else {
            List::col([
                Text::new(title, title_style)
                    .wrap(Wrap::BreakWords)
            ])
        }
            .draw(buf, rect.margin_left(checkbox_rect.width + 1));

        let todo_rect = rect.with_height(text_rect.height);

        if is_cur {
            Clear::new(clear_style.bold(true))
                .draw(buf, todo_rect.with_height(1));
        }

        if rect.height <= todo_rect.height {
            return todo_rect;
        }

        let todo_rect = rect.with_height(text_rect.height + 1);
        // Draw a line at the bottom
        let line_rect = rect
            .with_y(text_rect.y + 1)
            .with_height(text_rect.height)
            .bottom_border();

        Line::horizontal(BorderKind::Single)
            .style(Color::LightBlack)
            .draw(buf, line_rect);

        todo_rect
    }
}

/// Todo list
pub struct TodoList<'a>(pub &'a Vec<Todo>, pub usize);
impl<'a> Draw for TodoList<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let todos = self.0;
        let cur_todo = self.1;

        if todos.len() == 0 {
            return List::col([
                Text::from("no todos...")
                    .gray()
                    .italic()
                    .align(Align::Center),
                Text::from("[n] - to create a new todo")
                    .gray()
                    .italic()
                    .align(Align::Center)
            ]).draw(buf, rect.with_height(2).align_center(rect))
        }

        let mut height = 0u16;
        let scroll = cur_todo.saturating_sub(rect.height as usize / 6);

        for (index, todo) in todos.iter().enumerate() {
            let todo_rect = rect.margin((0, height, 0, 0));
            if todo_rect.height == 0 || scroll > index {
                continue;
            }

            let todo_rect = TodoWidget(todo, index == cur_todo)
                .draw(buf, todo_rect);

            height += todo_rect.height;
        }

        rect.with_height(height)
    }
}
