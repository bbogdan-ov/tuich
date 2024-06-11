#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature="text-span")]
use crate::text::{Line, Span, SplitSpans};

/// Wrap
/// Dictates how to wrap a text on overflow
///
/// # Notes
///
/// - Emojies are often displayed as 2 chars, but [Wrap] counts them as 1 char
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(Deserialize, Serialize))]
pub enum Wrap {
    /// Let text overflow
    #[default]
    None,
    /// Wrap words
    /// If the word is longer than `max_width` it will overflow
    Words,
    /// Wrap characters
    Break,
    /// Wrap words
    /// If the word is longer than `max_width` it will be breaked
    BreakWords
}
impl Wrap {
    pub fn calc<S: AsRef<str>>(
        &self,
        string: S,
        max_width: usize,
        max_lines: Option<usize>,
        first_indent: usize,
        indent: usize
    ) -> Vec<String> {
        wrap_str(self, vec![], string, max_width, max_lines, first_indent, first_indent, indent)
    }

    /// Wrap vector of spans
    /// Returns list of [Line], which contains list of [Span] and total line width *(number of graphemes)*
    ///
    /// # Arguments
    ///
    /// - `spans` - a list of spans to wrap
    /// - `max_width` - max width of the "box" in which the spans will be placed
    /// - `max_lines` - max number of wrapped lines. It can improve performance for for large amounts of text
    /// - `first_indent` - first line indentation (only for first line `max_width - first_indent`)
    /// - `indent` - indentation of all lines, except the first one (only for these lines `max_width - indent`)
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::{layout::*, style::*, text::*};
    /// let text: Vec<Span> = vec![
    ///     ("The funny thing about living ", Color::Red).into(),
    ///     "is nobody could do it ".into(),
    ///     ("alone...", Style::default().italic(true)).into(),
    /// ];
    ///
    /// assert_eq!(
    ///     Wrap::Words.calc_spans(text, 20, None, 0, 0),
    ///     vec![
    ///         Line::new(vec![
    ///             ("The funny thing", Color::Red).into()
    ///         ]),
    ///         Line::new(vec![
    ///             ("about living ", Color::Red).into(),
    ///             "is".into()
    ///         ]),
    ///         Line::new(vec![
    ///             "nobody could do it".into(),
    ///         ]),
    ///         Line::new(vec![
    ///             ("alone...", Style::default().italic(true)).into(),
    ///         ]),
    ///     ]
    /// );
    /// ```
    #[cfg(feature="text-span")]
    pub fn calc_spans<'a, S: AsRef<[Span<'a>]>>(
        &self,
        spans: S,
        max_width: usize,
        max_lines: Option<usize>,
        first_indent: usize,
        indent: usize
    ) -> Vec<Line<'a>> {
        wrap_spans(self, vec![], spans, max_width, max_lines, first_indent, first_indent, indent)
    }
}

#[cfg(feature="text-span")]
fn wrap_spans<'a, S: AsRef<[Span<'a>]>>(
    kind: &Wrap,
    mut lines: Vec<Line<'a>>,
    spans: S,
    max_width: usize,
    max_lines: Option<usize>,
    cur_indent: usize,
    first_indent: usize,
    indent: usize,
) -> Vec<Line<'a>> {
    let spans = spans.as_ref().to_vec();

    // Return if the number of lines is greater than allowed number
    if max_lines.is_some_and(|l| lines.len() >= l) {
        return lines;
    }

    // Return if wrap kind is None
    if kind.eq(&Wrap::None) {
        return if spans.len() > 0 {
            vec![Line::new(spans)]
        } else {
            vec![]
        }
    }

    let indented_max_width = max_width
        .saturating_sub(cur_indent)
        .max(1);

    // (span index, span's char index)
    let mut break_indice: Option<(usize, usize)> = None;
    let mut search_for_space = false;
    let mut total_width = 0usize;

    // Iterate through spans
    for (span_index, span) in spans.iter().enumerate() {
        // Iterate through span chars
        for char in span.content.char_indices() {
            total_width += 1;

            let cur_indice = (span_index, char.0);

            // If the total width of the spans (in chars) is greater than the max width...
            if total_width > indented_max_width {
                match kind {
                    Wrap::Words => {
                        if break_indice.is_some() {
                            // Space is already found, break
                            break;
                        } else {
                            // Space not found, keep searching...
                            search_for_space = true;
                        }
                    },
                    Wrap::BreakWords => {
                        if break_indice.is_none() {
                            // If there is no space, just break the span where we are now
                            break_indice = Some(cur_indice);
                        }
                        break;
                    },
                    Wrap::Break => {
                        // If break the span where we are now
                        break_indice = Some(cur_indice);
                        break;
                    },
                    Wrap::None => unreachable!()
                }
            }

            if char.1.eq(&' ') {
                break_indice = Some(cur_indice);

                if search_for_space {
                    break;
                }
            }
        }

        if total_width > indented_max_width && !search_for_space {
            break;
        }
    }

    // If line width is less than the max width or there is nothing to break, return what we already have
    if total_width <= indented_max_width || break_indice.is_none() {
        if spans.len() > 0 {
            lines.push(Line::new(spans));
        }
        return lines;
    }

    let break_indice = break_indice.unwrap();

    // Split line at given span index and span's char index
    let Some((left_line, right_line)) = spans.split_spans_at(break_indice.0, break_indice.1, kind.ne(&Wrap::Break)) else {
        return lines;
    };

    lines.push(left_line);

    wrap_spans(
        kind,
        lines,
        right_line.spans,
        max_width,
        max_lines,
        indent,
        first_indent,
        indent
    )
}

fn wrap_str<S: AsRef<str>>(
    kind: &Wrap,
    mut lines: Vec<String>,
    string: S,
    max_width: usize,
    max_lines: Option<usize>,
    cur_indent: usize,
    first_indent: usize,
    indent: usize,
) -> Vec<String> {
    let string = string.as_ref().to_string();

    // Return if the number of lines is greater than allowed number
    if max_lines.is_some_and(|l| lines.len() >= l) {
        return lines;
    }

    // Return if wrap kind is None
    if kind.eq(&Wrap::None) {
        return if string.is_empty() {
            vec![]
        } else {
            vec![string]
        }
    }

    let indented_max_width = max_width
        .saturating_sub(cur_indent)
        .max(1);

    let mut break_indice: Option<usize> = None;
    let mut search_for_space = false;
    let mut total_width = 0usize;

    for char in string.char_indices() {
        total_width += 1;

        let cur_indice = char.0;

        // If the line width is greater than the max width...
        if total_width > indented_max_width {
            match kind {
                Wrap::Words => {
                    if break_indice.is_some() {
                        // Space is already found, break
                        break;
                    } else {
                        // Space not found, keep searching...
                        search_for_space = true;
                    }
                },
                Wrap::BreakWords => {
                    if break_indice.is_none() {
                        // If there is no space, just break the span where we are now
                        break_indice = Some(cur_indice);
                    }
                    break;
                },
                Wrap::Break => {
                    // If break the span where we are now
                    break_indice = Some(cur_indice);
                    break;
                },
                Wrap::None => unreachable!()
            }
        }

        if char.1.eq(&' ') {
            break_indice = Some(cur_indice);

            if search_for_space {
                break;
            }
        }
    }

    // If line width is less than the max width or there is nothing to break, return what we already have
    if total_width <= indented_max_width || break_indice.is_none() {
        if !string.is_empty() {
            lines.push(string);
        }
        return lines;
    }

    let break_indice = break_indice.unwrap();

    let trim = kind.ne(&Wrap::Break);

    let left_line =
        if trim { string[..break_indice].trim_end() }
        else { &string[..break_indice] }
        .to_string();
    let right_line =
        if trim { string[break_indice..].trim_start() }
        else { &string[break_indice..] }
        .to_string();

    lines.push(left_line);

    wrap_str(
        kind,
        lines,
        right_line,
        max_width,
        max_lines,
        indent,
        first_indent,
        indent
    )
}

// Tests
#[cfg(feature="text-span")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping() {
        // TODO: add tests with styled spans

        let a: Vec<Span>      = vec!["😄The funny 😄thing about 😄living 😄is nobody could do it😄 alone...".into()];
        let a_long: Vec<Span> = vec!["The funnythingaboutlivingis nobody could do it alone...".into()];
        let b: Vec<Span>      = vec!["𝓽𝓱𝓲𝓼 𝓽𝓮𝔁𝓽 𝔀𝓪𝓼 𝔀𝓻𝓲𝓽𝓽𝓮𝓷 𝓽𝓸 𝓽𝓮𝓼𝓽 𝓽𝓮𝔁𝓽 𝔀𝓻𝓪𝓹𝓹𝓲𝓷𝓰 𝓲𝓷 𝓽𝓾𝓲𝓬𝓱!".into()];

        assert_eq!(Wrap::Words.calc_spans(&a, 20, None, 0, 0), vec![
            "😄The funny 😄thing".into(),
            "about 😄living 😄is".into(),
            "nobody could do it😄".into(),
            "alone...".into(),
        ], "Words wrap A");
        assert_eq!(Wrap::Break.calc_spans(&a, 20, None, 0, 0), vec![
            "😄The funny 😄thing ab".into(),
            "out 😄living 😄is nobo".into(),
            "dy could do it😄 alon".into(),
            "e...".into(),
        ], "Break wrap A");

        assert_eq!(Wrap::Words.calc_spans(&a_long, 20, None, 0, 0), vec![
            "The".into(),
            "funnythingaboutlivingis".into(),
            "nobody could do it".into(),
            "alone...".into(),
        ], "Words wrap A long");
        assert_eq!(Wrap::BreakWords.calc_spans(&a_long, 20, None, 0, 0), vec![
            "The".into(),
            "funnythingaboutlivin".into(),
            "gis nobody could do".into(),
            "it alone...".into(),
        ], "Break words wrap A long");

        assert_eq!(Wrap::Words.calc_spans(&b, 20, None, 0, 0), vec![
            "𝓽𝓱𝓲𝓼 𝓽𝓮𝔁𝓽 𝔀𝓪𝓼".into(),
            "𝔀𝓻𝓲𝓽𝓽𝓮𝓷 𝓽𝓸 𝓽𝓮𝓼𝓽".into(),
            "𝓽𝓮𝔁𝓽 𝔀𝓻𝓪𝓹𝓹𝓲𝓷𝓰 𝓲𝓷".into(),
            "𝓽𝓾𝓲𝓬𝓱!".into(),
        ], "Words wrap B");
        assert_eq!(Wrap::Break.calc_spans(&b, 20, None, 0, 0), vec![
            "𝓽𝓱𝓲𝓼 𝓽𝓮𝔁𝓽 𝔀𝓪𝓼 𝔀𝓻𝓲𝓽𝓽𝓮".into(),
            "𝓷 𝓽𝓸 𝓽𝓮𝓼𝓽 𝓽𝓮𝔁𝓽 𝔀𝓻𝓪𝓹𝓹".into(),
            "𝓲𝓷𝓰 𝓲𝓷 𝓽𝓾𝓲𝓬𝓱!".into(),
        ], "Break wrap B");

        assert_eq!(Wrap::Break.calc_spans(["small text".into()], 20, None, 0, 0), vec![
            "small text".into(),
        ], "Break wrap small");
        assert_eq!(Wrap::Words.calc_spans(["small text".into()], 20, None, 0, 0), vec![
            "small text".into(),
        ], "Words wrap small");
    }
}
