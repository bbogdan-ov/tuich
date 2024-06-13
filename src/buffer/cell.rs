use compact_str::CompactString;
use unicode_width::UnicodeWidthStr;

use crate::style::{Color, Style};

/// Cell
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Cell {
    pub char: Option<CompactString>,
    pub style: Style,
    display_width: usize
}
impl Cell {
    pub fn new<C, S>(char: Option<C>, style: S) -> Self
    where C: Into<CompactString>,
          S: Into<Style>
    {
        let (char, display_width) = if let Some(char) = char {
            let char: CompactString = char.into();
            let width = char.width();
            (Some(char), width)
        } else {
            (None, 0)
        };

        Self {
            char: char.into(),
            style: style.into(),
            display_width
        }
    }
    pub fn clear() -> Self {
        Self {
            char: Some(" ".into()),
            style: Style::clear(),
            display_width: 1
        }
    }

    //

    pub fn set_cell<C: Into<Cell>>(&mut self, cell: C) {
        let cell: Cell = cell.into();
        self.set(cell.char, cell.style)
    }
    pub fn set<C, S>(&mut self, char: Option<C>, style: S)
    where C: Into<CompactString>,
          S: Into<Style>
    {
        self.set_char(char);
        self.set_style(style);
    }
    pub fn set_char<C: Into<CompactString>>(&mut self, char: Option<C>) {
        if let Some(char) = char {
            let char: CompactString = char.into();
            self.display_width = char.width();
            self.char = Some(char);
        }
    }
    pub fn set_style<S: Into<Style>>(&mut self, style: S) {
        self.style = self.style.set(style.into());
    }

    pub fn display_width(&self) -> usize {
        self.display_width
    }
}
impl From<Style> for Cell {
    fn from(value: Style) -> Self {
        Self::new(Some(" "), value)
    }
}
impl From<Color> for Cell {
    fn from(value: Color) -> Self {
        Self::new(Some(" "), Style::default().bg(value))
    }
}
impl<C, S> From<(C, S)> for Cell
where C: Into<CompactString>,
      S: Into<Style>
{
    fn from(value: (C, S)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}
