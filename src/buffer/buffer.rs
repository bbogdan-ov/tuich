use compact_str::CompactString;
use unicode_segmentation::UnicodeSegmentation;

use crate::{layout::{Point, Rect}, style::Style, widget::RefDraw};

use super::Cell;

/// Buffer
#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>
}
impl Buffer {
    pub fn new(width: u16, height: u16, cells: Vec<Cell>) -> Self {
        Self {
            width,
            height,
            cells
        }
    }
    /// Creates a [Buffer] filled with `cell`
    pub fn filled(width: u16, height: u16, cell: &Cell) -> Self {
        let mut buf = Self::new(width, height, vec![]);
        buf.fill_with(cell);
        buf
    }
    /// Creates a [Buffer] filled with empty [Cell]
    pub fn empty(width: u16, height: u16) -> Self {
        Self::filled(width, height, &Cell::clear())
    }

    //

    /// Replace all the cells in the buffer with `cell`
    pub fn fill_with(&mut self, cell: &Cell) {
        let cells_len = self.width * self.height;
        if cells_len as usize != self.cells.len() {
            // If number of cells has changed, remove all and generate with new buffer size
            self.cells.clear();
            for _ in 0..cells_len {
                self.cells.push(cell.clone());
            }
        } else {
            // If number of cells hasn't changed, just update all cells
            for c in &mut self.cells {
                c.set_cell(cell.clone());
            }
        }
    }
    /// Clear/reset all cells
    pub fn clear(&mut self) {
        self.fill_with(&Cell::clear());
    }
    /// Clear the buffer and set size
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.clear();
    }
    
    /// Set a string with limited width in some position
    /// Returns the width of this string
    pub fn set_clamped_string<T, S>(&mut self, pos: (u16, u16), offset: u16, string: T, style: S, max_width: u16) -> u16
    where T: AsRef<str>,
          S: Into<Style>
    {
        let string = string.as_ref();
        let style: Style = style.into();

        let mut width = 0u16;

        for char in string.graphemes(true).skip(offset as usize) {
            if width >= max_width {
                break;
            }

            self.set(pos.add((width, 0)), Some(char), style);
            width = width.saturating_add(1);
        }

        width
    }
    /// Set a string in some position
    /// Returns the width of this string
    pub fn set_string<T, S>(&mut self, pos: (u16, u16), offset: u16, string: T, style: S) -> u16
    where T: AsRef<str>,
          S: Into<Style>
    {
        self.set_clamped_string(pos, offset, string, style, self.width)
    }

    /// Set cell char and style in some position
    /// Returns successfully or not
    pub fn set<C, S>(&mut self, pos: (u16, u16), char: Option<C>, style: S) -> bool
    where C: Into<CompactString>,
          S: Into<Style>
    {
        if let Some(cell) = self.get_mut(pos) {
            cell.set(char, style);
            true
        } else {
            false
        }
    }
    /// Set cell in some position
    /// Same as [Buffer::set], but using [Cell]
    /// Returns successfully or not
    pub fn set_cell<C: Into<Cell>>(&mut self, pos: (u16, u16), cell: C) -> bool {
        if let Some(c) = self.get_mut(pos) {
            c.set_cell(cell);
            true
        } else {
            false
        }
    }
    /// Set cell char in some position
    /// Returns successfully or not
    pub fn set_char<C: Into<CompactString>>(&mut self, pos: (u16, u16), char: Option<C>) -> bool {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_char(char);
            true
        } else {
            false
        }
    }
    /// Set cell style in some position
    /// Returns successfully or not
    pub fn set_style<S: Into<Style>>(&mut self, pos: (u16, u16), style: S) -> bool {
        if let Some(cell) = self.get_mut(pos) {
            cell.set_style(style);
            true
        } else {
            false
        }
    }

    /// Get a [Cell] ref in a specific position, if exists
    /// Returns `None` if `x` or `y` is outside the buffer
    pub fn get(&self, pos: (u16, u16)) -> Option<&Cell> {
        let index = self.index_of(pos)?;
        self.cells.get(index)
    }
    /// Get a mut [Cell] ref in a specific position, if exists
    /// Returns `None` if `x` or `y` is outside the buffer
    pub fn get_mut(&mut self, pos: (u16, u16)) -> Option<&mut Cell> {
        let index = self.index_of(pos)?;
        self.cells.get_mut(index)
    }

    /// Convert the cell position to an index
    /// Returns `None` if `x` or `y` is outside the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::buffer::*;
    /// let buf = Buffer::empty(3, 2);
    ///
    /// assert_eq!(buf.index_of((0, 0)), Some(0));
    /// assert_eq!(buf.index_of((2, 0)), Some(2));
    /// assert_eq!(buf.index_of((2, 1)), Some(5));
    /// assert_eq!(buf.index_of((4, 0)), None, "Outside the buffer!");
    /// ```
    pub fn index_of(&self, pos: (u16, u16)) -> Option<usize> {
        if pos.x() >= self.width || pos.y() >= self.height {
            None
        } else {
            Some((pos.x() + pos.y() * self.width) as usize)
        }
    }
    /// Convert the cell index to a position `(x, y)`
    /// Returns `None` if `index` is greater than the number of buffer cells
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::{buffer::*, layout::*};
    /// let buf = Buffer::empty(3, 2);
    ///
    /// assert_eq!(buf.pos_of(1), Some((1, 0)));
    /// assert_eq!(buf.pos_of(3), Some((0, 1)));
    /// assert_eq!(buf.pos_of(5), Some((2, 1)));
    /// assert_eq!(buf.pos_of(10), None, "Outside the buffer!");
    /// ```
    pub fn pos_of(&self, index: usize) -> Option<(u16, u16)> {
        if index >= self.cells.len() {
            None
        } else {
            Some((index as u16 % self.width, index as u16 / self.width).into())
        }
    }

    /// Get buffer size `(width, height)`
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    /// Get buffer rect
    pub fn rect(&self) -> Rect {
        self.size().into()
    }
}
impl RefDraw for Buffer {
    /// Marge this buffer with another
    /// 
    /// # Notes
    ///
    /// - this buffer will be cropped if `rect` is smaller than rect of this buffer
    fn draw(&self, buf: &mut Buffer, rect: Rect) -> Rect {
        let rect = self.rect().min(rect);

        for y in 0..rect.height {
            for x in 0..rect.width {
                if let Some(cell) = self.get((x, y)) {
                    buf.set_cell((rect.x + x, rect.y + y), cell.clone());
                }
            }
        }

        rect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_buffer() {
        let mut buf = Buffer::empty(2, 2);

        assert_eq!(buf.cells.len(), 4, "Before resize");

        buf.resize(3, 3);

        assert_eq!(buf.cells.len(), 9, "After resize");
    }
}
