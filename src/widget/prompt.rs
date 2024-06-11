use crate::{buffer::{Buffer, Cell}, layout::Rect, state::PromptState, style::{BorderKind, Color, Style}};

use super::{Borders, Draw};

/// Prompt widget
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt<'a> {
    pub state: &'a PromptState,
    pub style: Style,
    pub cursor_style: Style,
    pub focused: bool,
    pub borders: Option<Borders>
}
impl<'a> Prompt<'a> {
    pub fn new(state: &'a PromptState) -> Self {
        Self {
            state,
            style: Style::default(),
            cursor_style: Style::new(Color::Reset, Color::Reset).reverse(true),
            focused: true,
            borders: None
        }
    }

    //

    /// Set prompt text style
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    /// Set prompt cursor style
    pub fn cursor_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.cursor_style = style.into();
        self
    }
    /// Set whether the cursor is visible or not
    pub fn focused(mut self, value: bool) -> Self {
        self.focused = value;
        self
    }
    /// Set border kind
    pub fn border_kind(mut self, kind: BorderKind) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.kind(kind));
        } else {
            self.borders = Some(Borders::new(kind));
        }

        self
    }
    /// Set border style
    pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.style(style));
        } else {
            self.borders = Some(Borders::single().style(style));
        }

        self
    }
    /// Set a cell to fill the background
    pub fn fill<C: Into<Cell>>(mut self, cell: C) -> Self {
        if let Some(borders) = self.borders {
            self.borders = Some(borders.fill(cell));
        } else {
            self.borders = Some(Borders::single().fill(cell));
        }

        self
    }
}

impl<'a> Draw for Prompt<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let is_borders = self.borders.is_some();
        let borders_rect = if let Some(borders) = self.borders {
            borders
                .draw(buf, rect.with_height(3))
                .margin(1)
        } else {
            rect.with_height(1)
        };

        let cur_pos = self.state.cursor_pos() as u16;
        let scroll =
            if self.focused { cur_pos.saturating_sub(borders_rect.sub_width(borders_rect.width/4).width) }
            else { 0 };

        // Draw text
        buf.set_clamped_string(
            (borders_rect.x, borders_rect.y),
            scroll,
            self.state.value(),
            self.style,
            borders_rect.width
        );

        // Draw cursor
        if self.focused {
            let scrolled_pos = cur_pos.saturating_sub(scroll);
            let cur_x = borders_rect.x.saturating_add(scrolled_pos);

            //if scrolled_pos < borders_rect.width {
                buf.set_style(
                    (cur_x, borders_rect.y),
                    self.cursor_style
                );
            //}
        }

        if is_borders {
            rect.with_height(3)
        } else {
            rect.with_height(1)
        }
    }
}
