use crate::{buffer::Buffer, layout::{Direction, Rect}};

use super::Draw;

/// List
/// Place items in a row or a column
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<W> {
    pub content: Vec<W>,
    pub direction: Direction,
    pub gap: u16,
}
impl<W: Draw> List<W> {
    pub fn new<C: Into<Vec<W>>>(content: C, direction: Direction) -> Self {
        Self {
            content: content.into(),
            direction,
            gap: 0,
        }
    }
    /// Creates a horizontal [List]
    pub fn row<C: Into<Vec<W>>>(content: C) -> Self {
        Self::new(content, Direction::Horizontal)
    }
    /// Creates a vertical [List]
    pub fn col<C: Into<Vec<W>>>(content: C) -> Self {
        Self::new(content, Direction::Vertical)
    }

    //

    /// Set items flow direction
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    /// Set gap between items
    pub fn gap(mut self, value: u16) -> Self {
        self.gap = value;
        self
    }
}

impl<W: Draw> Draw for List<W> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let mut max_width = 0u16;
        let mut max_height = 0u16;
        let mut size = 0u16;

        for item in self.content {
            let rect = match self.direction {
                Direction::Horizontal => rect.margin((size, 0, 0, 0)),
                Direction::Vertical => rect.margin((0, size, 0, 0))
            };
            if rect.width == 0 || rect.height == 0 {
                break;
            }

            let item_rect = item.draw(buf, rect);

            max_width = item_rect.width.max(max_width);
            max_height = item_rect.height.max(max_height);

            let add_size = match self.direction {
                Direction::Horizontal => item_rect.width,
                Direction::Vertical => item_rect.height,
            };

            size = size
                .saturating_add(add_size)
                .saturating_add(self.gap);
        }

        match self.direction {
            Direction::Horizontal => rect.with_width(size).with_height(max_height),
            Direction::Vertical => rect.with_height(size).with_width(max_width)
        }
    }
}
