use std::borrow::Cow;

use crate::{buffer::Buffer, layout::{Align, Rect}, style::Style, text::{Span, Text}, widget::Draw};

#[cfg(not(feature="text-wrap"))]
use crate::text::SpansWidth;

#[cfg(feature="text-wrap")]
use crate::layout::Wrap;

/// Paragraph
/// Draws complex text with stylized pieces (spans) and text wrapping
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Paragraph<'a> {
    pub spans: Vec<Span<'a>>,
    pub align: Align,
    #[cfg(feature="text-wrap")]
    pub wrap: Wrap,
    pub first_indent: u16,
    #[cfg(feature="text-wrap")]
    pub indent: u16
}
impl<'a> Paragraph<'a> {
    /// Creates [Text] from spans
    pub fn new<S: Into<Vec<Span<'a>>>>(spans: S) -> Self {
        Self {
            spans: spans.into(),
            align: Align::Start,
            #[cfg(feature="text-wrap")]
            wrap: Wrap::None,
            first_indent: 0,
            #[cfg(feature="text-wrap")]
            indent: 0,
        }
    }
    /// Creates plain [Paragraph] with single span
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::{widget::*, style::*, text::*};
    /// let p = Paragraph::plain("im a red span!", Color::Red);
    ///
    /// assert_eq!(
    ///     p,
    ///     Paragraph::new([
    ///         Span::new("im a red span!", Style::default().fg(Color::Red))
    ///     ])
    /// );
    /// ```
    pub fn plain<T, S>(text: T, style: S) -> Self
    where T: Into<Cow<'a, str>>,
          S: Into<Style>
    {
        Self::new([ (text, style).into() ])
    }
    /// Creates a vector of [Paragraph] lines
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::{widget::*, style::*};
    /// let a = Paragraph::lines("word 1 word 2", ());
    /// let b = Paragraph::lines("line 1\nline 2", Color::Red);
    ///
    /// assert_eq!(a, vec![ Paragraph::plain("word 1 word 2", ()) ]);
    /// assert_eq!(
    ///     b,
    ///     vec![
    ///         Paragraph::plain("line 1", Style::default().fg(Color::Red)),
    ///         Paragraph::plain("line 2", Style::default().fg(Color::Red))
    ///     ]
    /// );
    /// ```
    pub fn lines<T, S>(text: T, style: S) -> Vec<Self>
    where T: ToString,
          S: Into<Style>
    {
        let text = text.to_string();

        if text.contains('\n') {
            let style = style.into();
            let mut lines = vec![];

            for line in text.split('\n') {
                lines.push(Paragraph::plain(line.to_string(), style));
            }
            lines
        } else {
            vec![ Paragraph::plain(text, style) ]
        }
    }

    //

    /// Set wrap kind of the paragraph
    #[cfg(feature="text-wrap")]
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }
    /// Set lines horizontal alignment
    /// Acts like `text-align` in CSS
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    /// Set first line indentation
    /// It can also be known as a "break line" or "new paragraph"
    /// 
    /// It will look something like this:
    /// ```plain
    ///     First line indent is set to 4!
    /// And im a new line!
    /// Another line
    /// And so on...
    /// ```
    pub fn first_indent(mut self, value: u16) -> Self {
        self.first_indent = value;
        self
    }
    /// Set the indentation for the lines below the first one
    /// 
    /// It will look something like this:
    /// ```plain
    /// First line indent is 0
    ///     But the indent for the lines below is 4!
    ///     Another line
    ///     And so on...
    /// ```
    #[cfg(feature="text-wrap")]
    pub fn indent(mut self, value: u16) -> Self {
        self.indent = value;
        self
    }
}
impl<'a> From<Span<'a>> for Paragraph<'a> {
    fn from(value: Span<'a>) -> Self {
        Self::new([value])
    }
}
impl<'a> From<&[Span<'a>]> for Paragraph<'a> {
    fn from(value: &[Span<'a>]) -> Self {
        Self::new(value)
    }
}
impl<'a> From<Vec<Span<'a>>> for Paragraph<'a> {
    fn from(value: Vec<Span<'a>>) -> Self {
        Self::new(value)
    }
}
impl<'a> From<Text<'a>> for Paragraph<'a> {
    fn from(value: Text<'a>) -> Self {
        Self::plain(value.content, value.style)
    }
}
impl<'a, T, S> From<(T, S)> for Paragraph<'a>
where T: Into<Cow<'a, str>>,
      S: Into<Style>
{
    fn from(value: (T, S)) -> Self {
        Self::plain(value.0, value.1)
    }
}

impl<'a> Draw for Paragraph<'a> {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect {
        let x = rect.x;
        let y = rect.y;

        #[cfg(not(feature="text-wrap"))]
        {
            let line_width = self.spans.spans_width();
            let mut span_offset_x = self.first_indent
                .saturating_add(self.align.calc(line_width, rect.width as usize) as u16);

            for span in &self.spans {
                let span_x = x.saturating_add(span_offset_x);

                buf.set_string(
                    (span_x, y),
                    &span.content,
                    span.style
                );

                span_offset_x = span_offset_x.saturating_add(span.width() as u16);
            }

            rect.with_height(1)
        }

        #[cfg(feature="text-wrap")]
        {
            let max_width = rect.width;
            let mut height = 0u16;

            let lines = self.wrap.calc_spans(
                &self.spans,
                max_width as usize,
                Some(rect.height as usize),
                self.first_indent as usize,
                self.indent as usize,
            );

            for (line_index, line) in lines.iter().enumerate() {
                if line_index >= rect.height as usize {
                    return Rect::default();
                }

                let span_offset_x = 
                    if line_index == 0 { self.first_indent }
                    else { self.indent };
                let mut span_offset_x = span_offset_x
                    .min(max_width.saturating_sub(1))
                    .saturating_add(self.align.calc(line.width, max_width as usize) as u16);

                for span in &line.spans {
                    let span_x = x.saturating_add(span_offset_x);

                    buf.set_string(
                        (span_x, y + height),
                        0,
                        &span.content,
                        span.style
                    );

                    span_offset_x = span_offset_x.saturating_add(span.width() as u16);
                }

                height = height.saturating_add(1);
            }

            rect.with_height(height)
        }
    }
}
