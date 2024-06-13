use crate::{buffer::{Buffer, Cell}, layout::Rect};

use super::Draw;

/// Clear
/// Fill some area with a cell
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Clear {
    pub cell: Cell
}
impl Clear {
    pub fn new<C: Into<Cell>>(cell: C) -> Self {
        Self {
            cell: cell.into()
        }
    }
    /// Creates a [Clear] that fills an area with an empty [Cell]
    pub fn clear() -> Self {
        Self::new(Cell::cleared())
    }
}
impl Draw for Clear {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        for y in 0..rect.height {
            for x in 0..rect.width {
                buf.set_cell(
                    (rect.x.saturating_add(x), rect.y.saturating_add(y)),
                    self.cell.clone()
                );
            }
        }
        rect
    }
}
