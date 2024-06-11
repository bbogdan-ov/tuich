use super::{Span, SpansWidth};

/// Line
/// Line of spans
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
    /// Total spans width
    pub width: usize
}
impl<'a> Line<'a> {
    pub fn new(spans: Vec<Span<'a>>) -> Self {
        let width = spans.spans_width();
        Self { spans, width }
    }
}
impl<'a> From<&'a str> for Line<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(vec![ value.into() ])
    }
}

