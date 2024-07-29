use super::{Direction, Rect};

/// Stack
/// Useful for creating tables
/// Acts like `flex` property in CSS
///
/// # Example
///
/// ```no_run
/// let rect = Rect::new(0, 0, 100, 20);
/// let rects = Strack::row([3, 1, 2].as_ref())
///     .calc(rect);
///
/// Borders::single()
///     .draw(buf, rects[0]);
/// Borders::single()
///     .draw(buf, rects[1]);
/// Borders::single()
///     .draw(buf, rects[2]);
/// ```
///
/// This code will draw something like this:
///
/// ```plain
/// ┌───────────┐┌───┐┌───────┐
/// │           ││   ││       │
/// │     3     ││ 1 ││   2   │
/// │           ││   ││       │
/// └───────────┘└───┘└───────┘
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<'a> {
    lengths: &'a [u8],
    pub direction: Direction,
    pub gap: u16,
}
impl<'a> Stack<'a> {
    pub fn new(lengths: &'a [u8]) -> Self {
        Self {
            lengths,
            direction: Direction::Vertical,
            gap: 0,
        }
    }
    /// Creates a [Stack] with [Direction::Horizontal]
    pub fn row(lengths: &'a [u8]) -> Self {
        Self::new(lengths)
            .direction(Direction::Horizontal)
    }
    /// Creates a [Stack] with [Direction::Vertical]
    pub fn col(lengths: &'a [u8]) -> Self {
        Self::new(lengths)
            .direction(Direction::Vertical)
    }

    //

    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }
    /// Set a gap between rects
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    pub fn calc(self, rect: Rect) -> Vec<Rect> {
        if self.lengths.is_empty() {
            return vec![];
        }

        let mut rects = vec![];
        let side = match self.direction {
            Direction::Horizontal => rect.width as f32,
            Direction::Vertical => rect.height as f32
        };

        let gap = self.gap as f32;
        let mut offset = 0u16;
        let mut total_len: f32 = 0.0;

        for len in self.lengths {
            total_len += *len as f32 * side + gap;
        }

        for (index, len) in self.lengths.iter().enumerate() {
            let mut size = (side * (*len as f32 * side / total_len)) as u16;

            if index == self.lengths.len() - 1 {
                if offset + size < side as u16 {
                    size += side as u16 - (offset + size);
                } else if offset + size > side as u16 {
                    size -= (offset + size) - side as u16;
                }
            }

            rects.push(match self.direction {
                Direction::Horizontal => Rect::new(rect.x + offset, rect.y, size, rect.height),
                Direction::Vertical => Rect::new(rect.x, rect.y + offset, rect.width, size),
            });

            offset += size + self.gap;
        }

        rects
    }
}
