use crate::{buffer::Buffer, layout::{Direction, Rect, Side}, style::{BorderKind, Style}};

use super::Draw;

/// Line
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Line {
    pub kind: BorderKind,
    pub style: Style,
    pub direction: Direction,
    pub start: Option<char>,
    pub end: Option<char>,
}
impl Line {
    pub fn new(kind: BorderKind, direction: Direction) -> Self {
        Self {
            direction,
            kind,
            style: Style::default(),
            start: None,
            end: None,
        }
    }
    /// Creates an horizontal [Line]
    pub fn horizontal(kind: BorderKind) -> Self {
        Self::new(kind, Direction::Horizontal)
    }
    /// Creates an vertical [Line]
    pub fn vertical(kind: BorderKind) -> Self {
        Self::new(kind, Direction::Vertical)
    }

    //

    /// Set line border kind
    pub fn kind<B: Into<BorderKind>>(mut self, kind: B) -> Self {
        self.kind = kind.into();
        self
    }
    /// Set line style
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    /// Set line orientation/direction
    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }
    /// Set a char at the left/top pole of the line
    pub fn start<V: Into<Option<char>>>(mut self, value: V) -> Self {
        self.start = value.into();
        self
    }
    /// Set a char at the right/bottom pole of the line
    pub fn end<V: Into<Option<char>>>(mut self, value: V) -> Self {
        self.end = value.into();
        self
    }
}

impl Draw for Line {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        match self.direction {
            Direction::Horizontal => {
                let char = self.kind.char_at(Side::Top);

                for cx in 0..rect.width {
                    buf.set(
                        (rect.x + cx, rect.y),
                        Some(char.to_string()),
                        self.style
                    );
                }

                if let Some(end) = self.end {
                    buf.set(
                        (rect.x + rect.width.saturating_sub(1), rect.y),
                        Some(end.to_string()),
                        self.style
                    );
                }
                if let Some(start) = self.start {
                    buf.set(
                        (rect.x, rect.y),
                        Some(start.to_string()),
                        self.style
                    );
                }

                rect.with_height(1)
            },
            Direction::Vertical => {
                let char = self.kind.char_at(Side::Left);

                for cy in 0..rect.height {
                    buf.set(
                        (rect.x, rect.y + cy),
                        Some(char.to_string()),
                        self.style
                    );
                }

                if let Some(end) = self.end {
                    buf.set(
                        (rect.x, rect.y + rect.height.saturating_sub(1)),
                        Some(end.to_string()),
                        self.style
                    );
                }
                if let Some(start) = self.start {
                    buf.set(
                        (rect.x, rect.y),
                        Some(start.to_string()),
                        self.style
                    );
                }

                rect.with_width(1)
            }
        }
    }
}
