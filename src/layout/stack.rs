use super::{Clamp, Direction, Rect};

/// Overflow
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Overflow {
    /// Do nothing on overflow
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌────────┐
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// └│ item │┘ - overflow occurres here!
    ///  └──────┘ 
    ///  ┌──────┐  - items are still being displayed...
    ///  │ item │
    ///  └──────┘
    #[default]
    None,
    /// Prevent from overflow
    /// For example, items in a list will be hidden when overflowing
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌────────┐
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │        │ - item was hidden
    /// └────────┘ - overflow occurres here!
    /// ```
    Prevent,
    /// Clip on overflow
    /// For example, items in a list will be truncated when overflowing
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌────────┐
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// └────────┘ - overflow occurres here!
    /// ```
    Clip,
    /// Allow overflow once, but prevent other overflows
    /// For example, if an overflow has already occurred in a list,
    /// it will not be allowed for other items
    ///
    /// # Example
    ///
    /// ```plain
    /// ┌────────┐
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// ││ item ││
    /// │└──────┘│
    /// │┌──────┐│
    /// └│ item │┘ - overflow occurres here!
    ///  └──────┘  - items below are hidden
    /// ```
    Skip,
}

/// Stack
/// Useful for creating layouts
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<'a> {
    lengths: &'a [Length],
    pub direction: Direction,
    pub gap: u16,
    pub overflow: Overflow
}
impl<'a> Stack<'a> {
    pub fn new(lengths: &'a [Length]) -> Self {
        Self {
            lengths,
            direction: Direction::Vertical,
            gap: 0,
            overflow: Overflow::Clip
        }
    }
    /// Creates a [Stack] with [Direction::Horizontal]
    pub fn row(lengths: &'a [Length]) -> Self {
        Self::new(lengths)
            .direction(Direction::Horizontal)
    }
    /// Creates a [Stack] with [Direction::Vertical]
    pub fn col(lengths: &'a [Length]) -> Self {
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
    /// Set behaviour on overflow
    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn calc(self, rect: Rect) -> Vec<Rect> {
        let mut rects = vec![];
        let side = match self.direction {
            Direction::Horizontal => rect.width,
            Direction::Vertical => rect.height
        };

        let lens_count = self.lengths.len();

        let mut offset = 0u16;
        let mut lens = Vec::<Length>::new();
        let mut fixed_len = 0u16;
        let mut max_len = 0u16;

        for i in 0..lens_count {
            let len = self.lengths[i].clamped(side);

            if let Length::Value(v) = len {
                fixed_len = fixed_len.saturating_add(v);
            } else {
                max_len = max_len.saturating_add(len.calc(side));
            }

            if i < lens_count.saturating_sub(1) {
                fixed_len = fixed_len.saturating_add(self.gap);
            }

            lens.push(len);
        }

        let max_len = max_len as f32;
        let remaining_side = side.saturating_sub(fixed_len) as f32;

        for len in lens {
            let mut size = if let Length::Value(v) = len {
                v
            } else {
                (len.calc(side) as f32 / max_len * remaining_side).round() as u16
            };

            let next_offset = offset.saturating_add(size); 

            match self.overflow {
                Overflow::None => (),
                Overflow::Clip => {
                    if offset > side {
                        break;
                    } else if next_offset > side {
                        size = size.saturating_sub(next_offset.saturating_sub(side));
                    }
                },
                Overflow::Prevent => {
                    if next_offset > side {
                        break;
                    }
                },
                Overflow::Skip => {
                    if offset > side {
                        break;
                    }
                }
            }

            let rect = match self.direction {
                Direction::Horizontal => Rect::new(rect.x.saturating_add(offset), rect.y, size, rect.height),
                Direction::Vertical => Rect::new(rect.x, rect.y.saturating_add(offset), rect.width, size),
            };

            offset = next_offset + self.gap;

            rects.push(rect);
        }

        rects
    }
}

/// Length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    /// Fill a remaining space (acts like [Length::Factor]`(100)`)
    Fill(Clamp),
    Factor(u8, Clamp),
    /// Fill certain amount of chars
    Value(u16),
}
impl Length {
    pub fn fill() -> Self {
        Self::Fill(Clamp::None)
    }
    pub fn value(chars: u16) -> Self {
        Self::Value(chars)
    }
    pub fn factor(percentage: u8) -> Self {
        Self::Factor(percentage, Clamp::None)
    }

    /// Set min value
    /// Do nothing if length is [Length::Value]
    pub fn min(self, min: u16) -> Self {
        match self {
            Self::Fill(c) => Self::Fill(c.set_min(min)),
            Self::Factor(p, c) => Self::Factor(p, c.set_min(min)),
            len => len
        }
    }
    /// Set max value
    /// Do nothing if length is [Length::Value]
    pub fn max(self, max: u16) -> Self {
        match self {
            Self::Fill(c) => Self::Fill(c.set_max(max)),
            Self::Factor(p, c) => Self::Factor(p, c.set_max(max)),
            len => len
        }
    }
    /// Set min and max value
    /// Do nothing if length is [Length::Value]
    pub fn clamp(self, min: u16, max: u16) -> Self {
        match self {
            Self::Fill(_) => Self::Fill(Clamp::MinMax(min, max)),
            Self::Factor(p, _) => Self::Factor(p, Clamp::MinMax(min, max)),
            len => len
        }
    }

    /// Applies [Clamp]
    /// Just returns this length if it is [Length::Value]
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let none = Length::Fill(Clamp::None);
    /// let min = Length::Fill(Clamp::Min(5));
    /// let max = Length::Fill(Clamp::Max(10));
    /// let min_max = Length::Fill(Clamp::MinMax(5, 10));
    ///
    /// let factor_max = Length::Factor(50, Clamp::Max(10)); // 50%
    ///
    /// assert_eq!(none.clamped(0), Length::Fill(Clamp::None));
    /// assert_eq!(none.clamped(100), Length::Fill(Clamp::None));
    ///
    /// assert_eq!(min.clamped(0), Length::Value(5));
    /// assert_eq!(min.clamped(20), Length::Fill(Clamp::None));
    ///
    /// assert_eq!(max.clamped(5), Length::Fill(Clamp::None));
    /// assert_eq!(max.clamped(15), Length::Value(10));
    ///
    /// assert_eq!(min_max.clamped(0), Length::Value(5));
    /// assert_eq!(min_max.clamped(9), Length::Fill(Clamp::None));
    /// assert_eq!(min_max.clamped(10), Length::Value(10));
    /// assert_eq!(min_max.clamped(15), Length::Value(10));
    ///
    /// assert_eq!(factor_max.clamped(15), Length::Factor(50, Clamp::None));
    /// assert_eq!(factor_max.clamped(24), Length::Value(10));
    /// ```
    pub fn clamped(&self, inside: u16) -> Self {
        match self {
            Self::Fill(clamp) => {
                let val = self.calc(inside);
                if !clamp.in_bounds(val) {
                    Self::Value(val)
                } else {
                    Self::Fill(Clamp::None)
                }
            },
            Self::Factor(p, clamp) => {
                let val = self.calc(inside);
                if !clamp.in_bounds(val) {
                    Self::Value(val)
                } else {
                    Self::Factor(*p, Clamp::None)
                }
            }
            Self::Value(v) => Self::Value(*v)
        }
    }

    /// Calculate length
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// assert_eq!(Length::Fill(Clamp::None).calc(10), 10);
    /// assert_eq!(Length::Fill(Clamp::Max(6)).calc(10), 6);
    ///
    /// assert_eq!(Length::Factor(50, Clamp::None).calc(10), 5); // 50%
    /// assert_eq!(Length::Factor(10, Clamp::None).calc(10), 1); // 10%
    /// assert_eq!(Length::Factor(200, Clamp::None).calc(10), 20); // 200%
    /// assert_eq!(Length::Factor(60, Clamp::Max(4)).calc(10), 4); // 60%
    ///
    /// assert_eq!(Length::Value(5).calc(10), 5);
    /// assert_eq!(Length::Value(100).calc(10), 100);
    /// ```
    pub fn calc(&self, inside: u16) -> u16 {
        match self {
            Self::Fill(clamp) => clamp.calc(inside),
            Self::Factor(persentage, clamp) => clamp.calc(
                (inside as f32 * (*persentage as f32 / 100.)) as u16
            ),
            Self::Value(v) => *v
        }
    }
}
impl Default for Length {
    fn default() -> Self {
        Self::Fill(Clamp::None)
    }
}
impl From<u16> for Length {
    fn from(value: u16) -> Self {
        Self::Value(value)
    }
}
impl From<Clamp> for Length {
    fn from(value: Clamp) -> Self {
        Self::Fill(value)
    }
}
impl<C: Into<Clamp>> From<(u8, C)> for Length {
    fn from(v: (u8, C)) -> Self {
        Self::Factor(v.0, v.1.into())
    }
}
