use std::borrow::Cow;

use crate::{buffer::{Buffer, Cell}, layout::Rect, style::{BorderKind, Style}, text::Text};
use super::{Borders, Draw};

/// Block
/// Borders with title
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<T, F> {
    pub title: Option<T>,
    pub title_margin: u16,
    pub footer: Option<F>,
    pub footer_margin: u16,
    pub kind: BorderKind,
    pub style: Style,
    pub fill: Option<Cell>
}
impl<'a, T: Draw, F: Draw> Block<T, F> {
    pub fn new(title: T) -> Self {
        Self {
            title: Some(title),
            title_margin: 1,
            footer: None,
            footer_margin: 1,
            kind: BorderKind::default(),
            style: Style::default(),
            fill: None
        }
    }

    //

    /// Set title text
    pub fn title<C: Into<T>>(mut self, title: C) -> Self {
        self.title = Some(title.into());
        self
    }
    /// Set title horizontal margin
    pub fn title_margin(mut self, value: u16) -> Self {
        self.title_margin = value;
        self
    }
    /// Set footer text
    pub fn footer<C: Into<F>>(mut self, footer: C) -> Self {
        self.footer = Some(footer.into());
        self
    }
    /// Set footer horizontal margin
    pub fn footer_margin(mut self, value: u16) -> Self {
        self.footer_margin = value;
        self
    }
    /// Set border kind
    pub fn kind(mut self, kind: BorderKind) -> Self {
        self.kind = kind;
        self
    }
    /// Set border style
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
impl<'a> Default for Block<Text<'a>, Text<'a>> {
    fn default() -> Self {
        Self {
            title: None,
            title_margin: 1,
            footer: None,
            footer_margin: 1,
            kind: BorderKind::default(),
            style: Style::default(),
            fill: None
        }
    }
}
impl<'a, F: Draw> From<Cow<'a, str>> for Block<Text<'a>, F> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::new(Text::from(value))
    }
}
impl<'a, F: Draw> From<&'a str> for Block<Text<'a>, F> {
    fn from(value: &'a str) -> Self {
        Self::new(Text::from(value))
    }
}
impl<'a, F: Draw> From<String> for Block<Text<'a>, F> {
    fn from(value: String) -> Self {
        Self::new(Text::from(value))
    }
}

impl<T: Draw, F: Draw> Draw for Block<T, F> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let borders = Borders::new(self.kind).style(self.style);

        let rect = if let Some(fill) = &self.fill {
            // Draw borders with fill
            borders
                .fill(fill.clone())
                .draw(buf, rect)
        } else {
            // Draw borders without fill
            borders.draw(buf, rect)
        };

        // Draw footer
        if rect.height > 1 {
            if let Some(footer) = self.footer {
                footer.draw(
                    buf,
                    rect.margin((self.footer_margin, 0))
                        .bottom_border()
                );
            }
        }

        // Draw title
        if let Some(title) = self.title {
            title.draw(
                buf,
                rect.margin((self.title_margin, 0))
                    .top_border()
            );
        }

        rect
    }
}
