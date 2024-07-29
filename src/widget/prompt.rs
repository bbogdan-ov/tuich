use crate::{
    buffer::{Buffer, Cell},
    layout::Rect,
    style::{BorderKind, Color, Style},
};

use super::{Borders, Draw};

use std::usize;

use stringslice::StringSlice;
use unicode_width::UnicodeWidthStr;

#[cfg(feature = "backend-event")]
use crate::event::Key;

/// Prompt message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptAction {
    /// Push char into prompt
    Char(char),
    /// Push string into prompt
    String(String),

    /// Delete chars to the left
    DeleteLeft(usize),
    /// Delete chars to the right
    DeleteRight(usize),
    /// Delete next word
    DeleteNextWord,
    /// Delete previous word
    DeletePrevWord,
    /// Delete all from the cursor pos to the start of prompt
    DeleteToStart,
    /// Delete all from the cursor pos to the end of prompt
    DeleteToEnd,
    /// Delete everything
    Clear,

    /// Move the cursor left
    MoveLeft(usize),
    /// Move the cursor right
    MoveRight(usize),
    /// Move the cursor to the end of a word
    MoveNextWord,
    /// Move the cursor to the start of a word
    MovePrevWord,
    /// Move the cursor to the start of the prompt
    MoveStart,
    /// Move the cursor to the end of the prompt
    MoveEnd,
    /// Move the cursor to the certain position in prompt
    MoveTo(usize),
}

/// Prompt state
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PromptState {
    value: String,
    cursor_pos: usize,
    width: usize,
}
impl PromptState {
    pub fn new<V: ToString>(value: V) -> Self {
        let value = value.to_string();
        let width = value.width();

        Self {
            value,
            cursor_pos: width,
            width,
        }
    }

    //

    /// Handle key events
    /// Uses most of the default bash keymaps! (Such as `Ctrl+U` to delete to the line start)
    /// Returns whether state has been updated or not
    #[cfg(feature = "backend-event")]
    pub fn handle_keys(&mut self, key: Key) -> bool {
        use self::PromptAction as Action;
        use crate::event::{KeyCode as C, KeyMod};

        const CTRL: KeyMod = KeyMod::Ctrl;
        const ALT: KeyMod = KeyMod::Alt;

        match key {
            Key(CTRL, C::Right) => self.action(Action::MoveNextWord),
            Key(CTRL, C::Left) => self.action(Action::MovePrevWord),
            Key(ALT, C::Char('f')) => self.action(Action::MoveNextWord),
            Key(ALT, C::Char('b')) => self.action(Action::MovePrevWord),
            Key(_, C::Left) => self.action(Action::MoveLeft(1)),
            Key(_, C::Right) => self.action(Action::MoveRight(1)),
            Key(CTRL, C::Char('b')) => self.action(Action::MoveLeft(1)),
            Key(CTRL, C::Char('f')) => self.action(Action::MoveRight(1)),
            Key(CTRL, C::Char('a')) => self.action(Action::MoveStart),
            Key(CTRL, C::Char('e')) => self.action(Action::MoveEnd),
            Key(_, C::Home) => self.action(Action::MoveStart),
            Key(_, C::End) => self.action(Action::MoveEnd),

            Key(CTRL, C::Char('w')) => self.action(Action::DeletePrevWord),
            // Same as Ctrl + Backspace
            Key(CTRL, C::Char('h')) => self.action(Action::DeletePrevWord),
            Key(CTRL, C::Char('u')) => self.action(Action::DeleteToStart),
            Key(CTRL, C::Char('k')) => self.action(Action::DeleteToEnd),
            Key(_, C::Backspace) => self.action(Action::DeleteLeft(1)),
            Key(_, C::Delete) => self.action(Action::DeleteRight(1)),

            Key(_, C::Char(char)) => self.action(Action::Char(char)),

            _ => false,
        }
    }

    /// Send a message to the state
    /// Returns whether state has been updated or not
    pub fn action(&mut self, msg: PromptAction) -> bool {
        use self::PromptAction as Msg;

        let cur = self.cursor_pos;

        match msg {
            Msg::Char(char) => self.push_char(cur, char),
            Msg::String(s) => self.push_string(cur, s),

            Msg::DeleteLeft(n) => self.delete_left(cur, n),
            Msg::DeleteRight(n) => self.delete_right(cur, n),
            Msg::DeleteNextWord => self.delete_next_word(cur),
            Msg::DeletePrevWord => self.delete_prev_word(cur),
            Msg::DeleteToStart => self.delete_to_start(cur),
            Msg::DeleteToEnd => self.delete_to_end(cur),
            Msg::Clear => self.clear(),

            Msg::MoveLeft(n) => self.move_left(n),
            Msg::MoveRight(n) => self.move_right(n),
            Msg::MoveNextWord => self.move_next_word(),
            Msg::MovePrevWord => self.move_prev_word(),
            Msg::MoveStart => self.move_start(),
            Msg::MoveEnd => self.move_end(),
            Msg::MoveTo(pos) => self.move_to(pos),
        }
    }

    // Pushing

    pub fn push_char(&mut self, pos: usize, char: char) -> bool {
        self.value = format!(
            "{}{char}{}",
            self.value.slice(..pos),
            self.value.slice(pos..)
        );
        self.calc_width();
        self.move_to(self.cursor_pos + 1)
    }
    pub fn push_string<S: ToString>(&mut self, pos: usize, s: S) -> bool {
        let s = s.to_string();
        let width = s.width();

        self.value = format!("{}{s}{}", self.value.slice(..pos), self.value.slice(pos..));
        self.calc_width();
        self.move_right(width)
    }

    // Deleting

    pub fn delete_left(&mut self, pos: usize, amount: usize) -> bool {
        self.value = format!(
            "{}{}",
            self.value.slice(..pos.saturating_sub(amount)),
            self.value.slice(pos..)
        );
        self.calc_width();
        self.move_left(amount)
    }
    pub fn delete_right(&mut self, pos: usize, amount: usize) -> bool {
        self.value = format!(
            "{}{}",
            self.value.slice(..pos),
            self.value.slice(pos.saturating_add(amount)..)
        );
        self.calc_width();
        true
    }
    pub fn delete_next_word(&mut self, pos: usize) -> bool {
        let w = self.get_next_word_width(pos);
        self.delete_right(pos, w);
        self.move_to(self.cursor_pos)
    }
    pub fn delete_prev_word(&mut self, pos: usize) -> bool {
        let w = self.get_prev_word_width(pos);
        self.delete_left(pos, w);
        self.move_to(self.cursor_pos)
    }
    pub fn delete_to_start(&mut self, pos: usize) -> bool {
        self.value = self.value.slice(pos..).to_string();
        self.calc_width();
        self.move_to(0)
    }
    pub fn delete_to_end(&mut self, pos: usize) -> bool {
        self.value = self.value.slice(..pos).to_string();
        self.calc_width();
        self.move_to(self.width)
    }
    pub fn clear(&mut self) -> bool {
        self.value = String::new();
        self.width = 0;
        self.cursor_pos = 0;
        true
    }

    // Move cursor

    pub fn move_start(&mut self) -> bool {
        self.move_to(0)
    }
    pub fn move_end(&mut self) -> bool {
        self.move_to(self.width)
    }
    pub fn move_next_word(&mut self) -> bool {
        let w = self.get_next_word_width(self.cursor_pos);
        self.move_right(w)
    }
    pub fn move_prev_word(&mut self) -> bool {
        let w = self.get_prev_word_width(self.cursor_pos);
        self.move_left(w)
    }
    pub fn move_left(&mut self, amount: usize) -> bool {
        self.move_to(self.cursor_pos.saturating_sub(amount))
    }
    pub fn move_right(&mut self, amount: usize) -> bool {
        self.move_to(self.cursor_pos + amount)
    }
    pub fn move_to(&mut self, pos: usize) -> bool {
        self.cursor_pos = pos.min(self.width);
        true
    }

    // Get

    pub fn char_at(&self, pos: usize) -> Option<&str> {
        self.value.try_slice(pos..pos + 1)
    }
    pub fn get_word_width<I>(&self, range: I) -> usize
    where
        I: Iterator,
        I::Item: Into<usize>,
    {
        let mut was_non_sep = false;
        let mut width = 0usize;

        for i in range {
            if self.char_at(i.into()).is_some_and(|c| c.eq(" ")) {
                if was_non_sep {
                    break;
                } else {
                    width += 1;
                }
            } else {
                was_non_sep = true;
                width += 1;
            }
        }

        width
    }
    pub fn get_next_word_width(&self, pos: usize) -> usize {
        self.get_word_width(pos..self.width)
    }
    pub fn get_prev_word_width(&self, pos: usize) -> usize {
        self.get_word_width((0..pos).rev())
    }

    fn calc_width(&mut self) {
        self.width = self.value.width();
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Prompt value
    pub fn value(&self) -> &String {
        &self.value
    }
    /// Cursor position
    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }
    /// Value width
    pub fn width(&self) -> usize {
        self.width
    }
}

/// Prompt widget
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt<'a> {
    pub state: &'a PromptState,
    pub style: Style,
    pub cursor_style: Style,
    pub focused: bool,
    pub borders: Option<Borders>
}
impl<'a> Prompt<'a> {
    pub fn new(state: &'a PromptState) -> Self {
        Self {
            state,
            style: Style::default(),
            cursor_style: Style::new(Color::Reset, Color::Reset).reverse(true),
            focused: true,
            borders: None,
        }
    }

    //

    /// Set prompt text style
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    /// Set prompt cursor style
    pub fn cursor_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.cursor_style = style.into();
        self
    }
    /// Set whether the cursor is visible or not
    pub fn focused(mut self, value: bool) -> Self {
        self.focused = value;
        self
    }
    /// Set border kind
    pub fn border_kind(mut self, kind: BorderKind) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.kind(kind));
        } else {
            self.borders = Some(Borders::new(kind));
        }

        self
    }
    /// Set border style
    pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.style(style));
        } else {
            self.borders = Some(Borders::single().style(style));
        }

        self
    }
    /// Set a cell to fill the background
    pub fn fill<C: Into<Cell>>(mut self, cell: C) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.fill(cell));
        } else {
            self.borders = Some(Borders::single().fill(cell));
        }

        self
    }
}

impl<'a> Draw for Prompt<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let is_borders = self.borders.is_some();
        let borders_rect = if let Some(borders) = self.borders {
            borders
                .draw(buf, rect.with_height(3))
                .margin(1)
        } else {
            rect.with_height(1)
        };

        let cur_pos = self.state.cursor_pos() as u16;
        let scroll = if self.focused {
            cur_pos.saturating_sub(borders_rect.sub_width(borders_rect.width / 4).width)
        } else {
            0
        };

        // Draw text
        buf.set_clamped_string(
            (borders_rect.x, borders_rect.y),
            scroll,
            self.state.value(),
            self.style,
            borders_rect.width
        );

        // Draw cursor
        if self.focused {
            let scrolled_pos = cur_pos.saturating_sub(scroll);
            let cur_x = borders_rect.x.saturating_add(scrolled_pos);

            buf.set_style((cur_x, borders_rect.y), self.cursor_style);
        }

        if is_borders {
            rect.with_height(3)
        } else {
            rect.with_height(1)
        }
    }
}
