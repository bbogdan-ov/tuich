use std::cmp::Ordering;

use crate::layout::Margin;

use super::{Align, AxisAlign};

/// Rect
/// The most important thing in ui layout building
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}
impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }
    /// Creates a [Rect] with specified `width` and `height`, but leaves position to `0, 0`
    pub fn sized(width: u16, height: u16) -> Self {
        Self::new(0, 0, width, height)
    }

    //

    /// Shrink the rect
    /// 
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let all = Rect::new(0, 0, 10, 10)
    ///     .margin(2); // add margin on all sides
    /// let axis = Rect::new(0, 0, 10, 10)
    ///     .margin((2, 4)); // add horizontal and vertical margins
    /// let each = Rect::new(0, 0, 10, 10)
    ///     .margin((1, 2, 3, 4)); // add margin on each side
    ///
    /// assert_eq!(all, Rect::new(2, 2, 6, 6), "All sides margin");
    /// assert_eq!(axis, Rect::new(2, 4, 6, 2), "Horizontal and vertical sides margin");
    /// assert_eq!(each, Rect::new(1, 2, 6, 4), "Each side margin");
    /// ```
    pub fn margin<M: Into<Margin>>(mut self, margin: M) -> Self {
        let margin: Margin = margin.into();

        self.x = self.x.saturating_add(margin.left());
        self.y = self.y.saturating_add(margin.top());
        self.width = self.width.saturating_sub(margin.right() + margin.left());
        self.height = self.height.saturating_sub(margin.bottom() + margin.top());

        self
    }
    /// Add margin to left
    pub fn margin_left(mut self, value: u16) -> Self {
        self.x = self.x.saturating_add(value);
        self.width = self.width.saturating_sub(value * 2);
        self
    }
    /// Add margin to top
    pub fn margin_top(mut self, value: u16) -> Self {
        self.y = self.y.saturating_add(value);
        self.height = self.height.saturating_sub(value * 2);
        self
    }
    /// Add margin to right
    pub fn margin_right(mut self, value: u16) -> Self {
        self.width = self.width.saturating_sub(value);
        self
    }
    /// Add margin to bottom
    pub fn margin_bottom(mut self, value: u16) -> Self {
        self.height = self.height.saturating_sub(value);
        self
    }

    /// Align this rect inside another
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let a = Rect::new(0, 0, 10, 8);
    /// let b = Rect::new(0, 0, 24, 30);
    ///
    /// assert_eq!(a.align(b, Align::Center), Rect::new(7, 11, 10, 8), "Center, center");
    /// assert_eq!(a.align(b, Align::Start), Rect::new(0, 0, 10, 8), "Left, top");
    /// assert_eq!(a.align(b, (Align::End, Align::Center)), Rect::new(14, 11, 10, 8), "Left, top");
    /// ```
    pub fn align<R, A>(self, inside: R, align: A) -> Self
    where R: Into<Rect>,
          A: Into<AxisAlign>
    {
        let inside: Rect = inside.into();
        let align: AxisAlign = align.into();

        let (x, y) = align.calc(self.size(), inside.size());

        self
            .with_x(x + inside.x)
            .with_y(y + inside.y)
    }
    /// Alias to [Rect::align] with [Align::Center]
    pub fn align_center<R: Into<Rect>>(self, inside: R) -> Self {
        self.align(inside, Align::Center)
    }

    pub fn clamp_size(mut self, min_size: (u16, u16), max_size: (u16, u16)) -> Self {
        self.width = self.width.clamp(min_size.0, max_size.0);
        self.height = self.height.clamp(min_size.1, max_size.1);
        self
    }
    pub fn min_size(mut self, min_size: (u16, u16)) -> Self {
        self.width = self.width.min(min_size.0);
        self.height = self.height.min(min_size.1);
        self
    }
    pub fn max_size(mut self, max_size: (u16, u16)) -> Self {
        self.width = self.width.max(max_size.0);
        self.height = self.height.max(max_size.1);
        self
    }

    /// Set `x` without modifying original rect
    pub fn with_x(mut self, x: u16) -> Self {
        self.x = x;
        self
    }
    /// Set `y` without modifying original rect
    pub fn with_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }
    /// Set `width` without modifying original rect
    pub fn with_width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
    /// Set `height` without modifying original rect
    pub fn with_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
    /// Set top left corner position without modifying original rect `(x, y)`
    pub fn with_pos(self, pos: (u16, u16)) -> Self {
        self.with_x(pos.0).with_y(pos.1)
    }
    /// Set size without modifying original rect `(width, height)`
    pub fn with_size(self, size: (u16, u16)) -> Self {
        self.with_width(size.0).with_height(size.1)
    }
    /// Set left without modifying original rect
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(2, 2, 10, 10);
    ///
    /// assert_eq!(r.with_left(4), Rect::new(4, 2, 8, 10));
    /// assert_eq!(r.with_left(12), Rect::new(12, 2, 0, 10));
    /// ```
    pub fn with_left(mut self, left: u16) -> Self {
        self.width = self.width.saturating_sub(left.saturating_sub(self.x));
        self.x = left;
        self
    }
    /// Set top without modifying original rect
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(2, 2, 10, 10);
    ///
    /// assert_eq!(r.with_top(4), Rect::new(2, 4, 10, 8));
    /// assert_eq!(r.with_top(12), Rect::new(2, 12, 10, 0));
    /// ```
    pub fn with_top(mut self, top: u16) -> Self {
        self.height = self.height.saturating_sub(top.saturating_sub(self.y));
        self.y = top;
        self
    }
    /// Set right without modifying original rect
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(2, 2, 10, 10);
    ///
    /// assert_eq!(r.with_right(4), Rect::new(2, 2, 2, 10));
    /// assert_eq!(r.with_right(14), Rect::new(2, 2, 12, 10));
    /// ```
    pub fn with_right(mut self, right: u16) -> Self {
        self.width = right.saturating_sub(self.x);
        self
    }
    /// Set bottom without modifying original rect
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(2, 2, 10, 10);
    ///
    /// assert_eq!(r.with_bottom(4), Rect::new(2, 2, 10, 2));
    /// assert_eq!(r.with_bottom(14), Rect::new(2, 2, 10, 12));
    /// ```
    pub fn with_bottom(mut self, top: u16) -> Self {
        self.height = top.saturating_sub(self.y);
        self
    }

    /// Add `value` to x
    pub fn add_x(mut self, value: u16) -> Self {
        self.x = self.x.saturating_add(value);
        self
    }
    /// Subtract `value` from x
    pub fn sub_x(mut self, value: u16) -> Self {
        self.x = self.x.saturating_sub(value);
        self
    }
    /// Add `value` to y
    pub fn add_y(mut self, value: u16) -> Self {
        self.y = self.y.saturating_add(value);
        self
    }
    /// Subtract `value` from y
    pub fn sub_y(mut self, value: u16) -> Self {
        self.y = self.y.saturating_sub(value);
        self
    }
    /// Add `value` to width
    pub fn add_width(mut self, value: u16) -> Self {
        self.width = self.width.saturating_add(value);
        self
    }
    /// Subtract `value` from width
    pub fn sub_width(mut self, value: u16) -> Self {
        self.width = self.width.saturating_sub(value);
        self
    }
    /// Add `value` to height
    pub fn add_height(mut self, value: u16) -> Self {
        self.height = self.height.saturating_add(value);
        self
    }
    /// Subtract `value` from height
    pub fn sub_height(mut self, value: u16) -> Self {
        self.height = self.height.saturating_sub(value);
        self
    }

    /// Get left side position (`x`)
    pub fn left(&self) -> u16 {
        self.x
    }
    /// Get top side position (`y`)
    pub fn top(&self) -> u16 {
        self.y
    }
    /// Get right side position (`x + width`)
    pub fn right(&self) -> u16 {
        self.x + self.width
    }
    /// Get bottom side position (`y + height`)
    pub fn bottom(&self) -> u16 {
        self.y + self.height
    }

    /// Get top left corner position `(x, y)`
    pub fn top_left(&self) -> (u16, u16) {
        (self.x, self.y)
    }
    /// Get top right corner position `(x + width - 1, y)`
    pub fn top_right(&self) -> (u16, u16) {
        (
            self.x
                .saturating_add(self.width)
                .saturating_sub(1),
            self.y
        )
    }
    /// Get bottom right corner position `(x + width - 1, y + height - 1)`
    pub fn bottom_right(&self) -> (u16, u16) {
        (
            self.x
                .saturating_add(self.width)
                .saturating_sub(1) ,
            self.y
                .saturating_add(self.height)
                .saturating_sub(1)
        )
    }
    /// Get bottom left corner position `(x, y + height - 1)`
    pub fn bottom_left(&self) -> (u16, u16) {
        (
            self.x,
            self.y
                .saturating_add(self.height)
                .saturating_sub(1)
        )
    }
    /// Get center position `(x + width / 2, y + height / 2)`
    pub fn center(&self) -> (u16, u16) {
        (
            self.x.saturating_add(self.width / 2),
            self.y.saturating_add(self.height / 2)
        )
    }

    /// Get top left corner position `(x, y)`
    pub fn pos(&self) -> (u16, u16) {
        self.top_left()
    }
    /// Get size `(width, height)`
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    /// Get area (`width * height`)
    pub fn area(&self) -> u16 {
        self.width.saturating_mul(self.height)
    }

    /// Returns left border rect `(x, y, 1, height)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(1, 2, 10, 20);
    ///
    /// assert_eq!(r.left_border(), Rect::new(1, 2, 1, 20));
    /// ```
    pub fn left_border(&self) -> Rect {
        Rect::new(self.x, self.y, 1, self.height)
    }
    /// Returns top border rect `(x, y, width, 1)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(1, 2, 10, 20);
    ///
    /// assert_eq!(r.top_border(), Rect::new(1, 2, 10, 1));
    /// ```
    pub fn top_border(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, 1)
    }
    /// Returns right border rect `(x + width - 1, y, 1, height)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(1, 2, 10, 20);
    ///
    /// assert_eq!(r.right_border(), Rect::new(10, 2, 1, 20));
    /// ```
    pub fn right_border(&self) -> Rect {
        Rect::new(
            self.x
                .saturating_add(self.width)
                .saturating_sub(1),
            self.y,
            1,
            self.height
        )
    }
    /// Returns bottom border rect `(x, y + height - 1, width, 1)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let r = Rect::new(1, 2, 10, 20);
    ///
    /// assert_eq!(r.bottom_border(), Rect::new(1, 21, 10, 1));
    /// ```
    pub fn bottom_border(&self) -> Rect {
        Rect::new(
            self.x,
            self.y
                .saturating_add(self.height)
                .saturating_sub(1),
            self.width,
            1
        )
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.area().cmp(&other.area()))
    }
}
impl Ord for Rect {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl From<(u16, u16)> for Rect {
    /// Creates a [Rect] with specified size `(width, height)`
    fn from(v: (u16, u16)) -> Self {
        Self::new(0, 0, v.0, v.1)
    }
}
impl From<(u16, u16, u16, u16)> for Rect {
    /// Creates a [Rect] with specified position and size `(x, y, width, height)`
    /// Same as `Rect::new(x, y, width, height)`
    fn from(v: (u16, u16, u16, u16)) -> Self {
        Self::new(v.0, v.1, v.2, v.3)
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_rect() {
        assert_eq!(Rect::from((10, 20)), Rect::new(0, 0, 10, 20));
        assert_eq!(Rect::from((1, 5, 10, 20)), Rect::new(1, 5, 10, 20));
    }
    #[test]
    fn rect_pos() {
        let r = Rect::new(1, 3, 10, 20);

        assert_eq!(r.left(), 1, "Left (x)");
        assert_eq!(r.top(), 3, "Top (y)");
        assert_eq!(r.right(), 11, "Right (x + width)");
        assert_eq!(r.bottom(), 23, "Bottom (y + height)");
        assert_eq!(r.top_left(), (1, 3), "Top left (x, y)");
        assert_eq!(r.top_right(), (10, 3), "Top right (x + width, y)");
        assert_eq!(r.bottom_right(), (10, 22), "Bottom right (x + width, y + height)");
        assert_eq!(r.bottom_left(), (1, 22), "Bottom left (x, y + height)");
        assert_eq!(r.pos(), (1, 3), "Position (x, y)");
        assert_eq!(r.size(), (10, 20), "Size (width, height)");
    }
}
