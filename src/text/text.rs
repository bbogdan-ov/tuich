use std::borrow::Cow;

use unicode_width::UnicodeWidthStr;

use crate::{buffer::Buffer, layout::{Align, Clip, Rect}, style::Style, widget::Draw};

#[cfg(feature="text-wrap")]
use crate::layout::Wrap;

#[cfg(feature="text-stylized")]
use crate::style::Stylized;

#[cfg(feature="text-span")]
use super::Span;

/// Text
/// Draws simple stylized text
/// Useful for drawing small pieces of text like buttons, block titles, etc.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Text<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
    pub align: Align,
    pub clip: Clip,
    pub clip_align: Align,
    #[cfg(feature="text-wrap")]
    pub wrap: Wrap,
}
impl<'a> Text<'a> {
    pub fn new<C, S>(content: C, style: S) -> Self
    where C: Into<Cow<'a, str>>,
          S: Into<Style>
    {
        Self {
            content: content.into(),
            style: style.into(),
            align: Align::Start,
            clip: Clip::Clip,
            clip_align: Align::End,
            #[cfg(feature="text-wrap")]
            wrap: Wrap::None,
        }
    }

    //

    /// Set text content
    pub fn content<C: Into<Cow<'a, str>>>(mut self, content: C) -> Self {
        self.content = content.into();
        self
    }
    /// Set text style
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }
    /// Set horizontal text alignment
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    /// Set text clip on overflow
    pub fn clip<C: Into<Clip>>(mut self, clip: C) -> Self {
        self.clip = clip.into();
        self
    }
    /// Set text clip alignment
    /// See [Clip::calc]
    pub fn clip_align(mut self, align: Align) -> Self {
        self.clip_align = align;
        self
    }
    /// Set text wrapping
    #[cfg(feature="text-wrap")]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }
}
impl<'a> From<Cow<'a, str>> for Text<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Self::new(value, ())
    }
}
impl<'a> From<String> for Text<'a> {
    fn from(value: String) -> Self {
        Self::new(value, ())
    }
}
impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value, ())
    }
}
#[cfg(feature="text-span")]
impl<'a> From<Span<'a>> for Text<'a> {
    fn from(value: Span<'a>) -> Self {
        Self::new(value.content, value.style)
    }
}
impl<'a, T, S> From<(T, S)> for Text<'a>
where T: Into<Cow<'a, str>>,
      S: Into<Style>
{
    fn from(value: (T, S)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(feature="text-stylized")]
impl<'a> Stylized for Text<'a> {
    type Output = Self;

    fn style<S: Into<Style>>(mut self, style: S) -> Self::Output {
        self.style = style.into();
        self
    }
    fn get_style(&self) -> Style {
        self.style
    }
}
impl<'a> Draw for Text<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {

        #[cfg(not(feature="text-wrap"))]
        {
            let content = self.clip.calc(&self.content, rect.width as usize, self.clip_align);
            let width = content.width();
            let x = self.align.calc(width, rect.width as usize) as u16;

            buf.set_string(
                (rect.x.saturating_add(x), rect.y),
                0,
                content,
                self.style
            );

            rect
                .with_width(width as u16)
                .with_height(1)
        }
        #[cfg(feature="text-wrap")]
        {
            let mut width = 0u16;
            let mut height = 0u16;
            let max_width = rect.width as usize;

            let lines = self.wrap.calc(
                &self.content,
                max_width,
                Some(rect.height as usize),
                0, 0
            );

            for line in lines {
                let content = self.clip.calc(&line, max_width, self.clip_align);
                let line_width = content.width();
                let x = self.align.calc(line_width, max_width) as u16;

                width = width.max(line_width as u16);

                buf.set_string(
                    (rect.x.saturating_add(x), rect.y.saturating_add(height)),
                    0,
                    content,
                    self.style
                );
                height = height.saturating_add(1);
            }

            if self.align.ne(&Align::Start) {
                width = rect.width;
            }

            rect
                .with_width(width)
                .with_height(height)
        }
    }
}
