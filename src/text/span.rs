use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;

use crate::{buffer::Buffer, layout::Rect, style::Style, widget::Draw};

use super::Line;

#[cfg(feature="text-stylized")]
use crate::style::Stylized;

// Traits
pub trait SpansWidth {
    /// Calculate total width of the spans in the vector
    fn spans_width(&self) -> usize;
}
impl<'a> SpansWidth for Vec<Span<'a>> {
    fn spans_width(&self) -> usize {
        self.iter().fold(0, |acc, s| acc + s.width())
    }
}

/// Split spans
pub trait SplitSpans<'a> {
    fn split_spans_at(&self, span_index: usize, char_index: usize, trim: bool) -> Option<(Line<'a>, Line<'a>)>;
    fn split_spans_at_char(&self, pos: usize, trim: bool) -> Option<(Line<'a>, Line<'a>)>;
}
impl<'a> SplitSpans<'a> for Vec<Span<'a>> {
    fn split_spans_at(&self, span_index: usize, char_index: usize, trim: bool) -> Option<(Line<'a>, Line<'a>)> {
        let mut left_spans = self[..span_index].to_vec();
        let mut right_spans =
            if span_index >= self.len() - 1 { vec![] }
            else { self[span_index+1..].to_vec() };

        let span_to_break = &self[span_index];
        let content = &span_to_break.content;

        let left_span_content =
            if trim { content[..char_index].trim_end() }
            else { &content[..char_index] }
            .to_string();

        let right_span_content =
            if trim { content[char_index..].trim_start() }
            else { &content[char_index..] }
            .to_string();

        if !left_span_content.is_empty() {
            left_spans.push(Span::new(
                left_span_content,
                span_to_break.style
            ));
        }
        if !right_span_content.is_empty() {
            right_spans.insert(0, Span::new(
                right_span_content,
                span_to_break.style
            ));
        }

        Some((Line::new(left_spans), Line::new(right_spans)))
    }
    fn split_spans_at_char(&self, pos: usize, trim: bool) -> Option<(Line<'a>, Line<'a>)> {
        let mut split_at_span: Option<usize> = None;
        let mut split_at_char: Option<usize> = None;

        let mut total_char_index = 0usize;
        for (span_index, span) in self.iter().enumerate() {
            for (char_index, _) in span.content.graphemes(true).enumerate() {
                if total_char_index == pos {
                    split_at_span = Some(span_index);
                    split_at_char = Some(char_index);
                }
                total_char_index += 1;
            }
        }

        self.split_spans_at(split_at_span?, split_at_char?, trim)
    }
}

/// Span
/// A stylized chuck of a larger text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
    
    width: usize
}
impl<'a> Span<'a> {
    pub fn new<C, S>(content: C, style: S) -> Self
    where C: Into<Cow<'a, str>>,
          S: Into<Style>
    {
        let content: Cow<'a, str> = content.into();
        let width = content.graphemes(true).count();
        Self {
            content,
            style: style.into(),
            width
        }
    }

    //

    /// Set [Span] content and recalculate it's width
    pub fn content<C: Into<Cow<'a, str>>>(mut self, value: C) -> Self {
        self.content = value.into();
        self.width = self.content.graphemes(true).count();
        self
    }

    /// Get content length
    /// Returns just number of bytes in the string ([str::len()])
    pub fn len(&self) -> usize {
        self.content.len()
    }
    /// Get content width
    /// Returns number of graphemes
    pub fn width(&self) -> usize {
        self.width
    }
}
impl<'a> From<&'a str> for Span<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value, ())
    }
}
impl<'a> From<String> for Span<'a> {
    fn from(value: String) -> Self {
        Self::new(value, ())
    }
}
impl<'a> From<char> for Span<'a> {
    fn from(value: char) -> Self {
        Self::new(value.to_string(), ())
    }
}
impl<'a, C, S> From<(C, S)> for Span<'a>
where C: Into<Cow<'a, str>>,
      S: Into<Style>
{
    fn from(value: (C, S)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(feature="text-stylized")]
impl<'a> Stylized for Span<'a> {
    type Output = Self;

    fn style<S: Into<Style>>(mut self, style: S) -> Self::Output {
        self.style = style.into();
        self
    }
    fn get_style(&self) -> Style {
        self.style
    }
}
impl<'a> Draw for Span<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        buf.set_string(rect.pos(), 0, &self.content, self.style);
        rect.with_height(1)
    }
}
