use crate::{buffer::{Buffer, Cell}, layout::{Rect, Side}, style::{BorderKind, Style}};

use super::{Clear, Draw, Line};

/// Borders
/// Draws a rectable with borders
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Borders {
    pub kind: BorderKind,
    pub style: Style,
    pub fill: Option<Cell>
}
impl Borders {
    pub fn new(kind: BorderKind) -> Self {
        Self {
            kind,
            style: Style::default(),
            fill: None
        }
    }
    /// Creates a [Borders] with [BorderKind::Single]
    pub fn single() -> Self {
        Self::new(BorderKind::Single)
    }
    /// Creates a [Borders] with [BorderKind::Double]
    pub fn double() -> Self {
        Self::new(BorderKind::Double)
    }
    /// Creates a [Borders] with [BorderKind::Rounded]
    pub fn rounded() -> Self {
        Self::new(BorderKind::Rounded)
    }
    /// Creates a [Borders] with [BorderKind::Thick]
    pub fn thick() -> Self {
        Self::new(BorderKind::Thick)
    }
    /// Creates a [Borders] with [BorderKind::Block]
    pub fn block() -> Self {
        Self::new(BorderKind::Block)
    }
    /// Creates a [Borders] with a custom border
    pub fn custom(chars: [char; 8]) -> Self {
        Self::new(BorderKind::Custom(chars))
    }

    //

    /// Set border kind
    pub fn kind(mut self, kind: BorderKind) -> Self {
        self.kind = kind;
        self
    }
    /// Set borders style
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    /// Set a cell to fill the background
    pub fn fill<C: Into<Cell>>(mut self, cell: C) -> Self {
        self.fill = Some(cell.into());
        self
    }
}

impl Draw for Borders {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let border_rect = rect;

        // Background
        if let Some(fill) = &self.fill {
            Clear::new(fill.clone())
                .draw(buf, rect.margin(1));
        }

        let ver_line = Line::vertical(self.kind)
            .style(self.style);
        let hor_line = Line::horizontal(self.kind)
            .style(self.style);

        // Lines
        if border_rect.height > 1 {
            // Left
            ver_line.clone().draw(buf, rect.left_border());
        }
        if border_rect.width > 1 {
            // Top
            hor_line.clone().draw(buf, rect.top_border());
        }

        if border_rect.width > 1 && border_rect.height > 1 {
            // Right
            ver_line.clone().draw(buf, border_rect.right_border());
            // Bottom
            hor_line.clone().draw(buf, border_rect.bottom_border());

            // Top left
            buf.set(
                border_rect.top_left(),
                Some(self.kind.string_at(Side::TopLeft)),
                self.style
            );
            // Top right
            buf.set(
                border_rect.top_right(),
                Some(self.kind.string_at(Side::TopRight)),
                self.style
            );
            // Bottom right
            buf.set(
                border_rect.bottom_right(),
                Some(self.kind.string_at(Side::BottomRight)),
                self.style
            );
            // Bottom left
            buf.set(
                border_rect.bottom_left(),
                Some(self.kind.string_at(Side::BottomLeft)),
                self.style
            );
        }

        rect
    }
}
