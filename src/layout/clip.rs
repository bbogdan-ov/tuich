use compact_str::CompactString;
use stringslice::StringSlice;
use unicode_width::UnicodeWidthStr;

use super::{Align, Place};

// TODO: add clipping for spans

/// Text clipping
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Clip {
    /// Do not trim the string
    #[default]
    None,
    /// Just trim the string on overflow
    Clip,
    /// Ellipsis (...)
    Ellipsis,
    /// Hide text on overflow
    Hide,
    Custom(CompactString)
}
impl Clip {
    /// Trim the string if its width is greater than `max_width`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let t = "im too long to fit in the width of 20 chars!";
    ///
    /// // Clip::None will not change the string
    /// assert_eq!(Clip::None.calc(t, 20, Align::End), t);
    ///
    /// // Align to end
    /// assert_eq!(Clip::Clip.calc(t, 20, Align::End),                "im too long to fit i");
    /// assert_eq!(Clip::Ellipsis.calc(t, 20, Align::End),            "im too long to fi...");
    /// assert_eq!(Clip::Custom("!!".into()).calc(t, 20, Align::End), "im too long to fit!!");
    ///
    /// // Align to start
    /// assert_eq!(Clip::Clip.calc(t, 20, Align::Start),                "e width of 20 chars!");
    /// assert_eq!(Clip::Ellipsis.calc(t, 20, Align::Start),            "...idth of 20 chars!");
    /// assert_eq!(Clip::Custom("!!".into()).calc(t, 20, Align::Start), "!!width of 20 chars!");
    ///
    /// // Align to center
    /// assert_eq!(Clip::Clip.calc(t, 20, Align::Center),                "im too lon 20 chars!");
    /// assert_eq!(Clip::Ellipsis.calc(t, 20, Align::Center),            "im too lo...0 chars!");
    /// assert_eq!(Clip::Ellipsis.calc(t, 29, Align::Center),            "im too long t... of 20 chars!");
    /// assert_eq!(Clip::Custom("!!".into()).calc(t, 20, Align::Center), "im too lo!!20 chars!");
    /// ```
    pub fn calc<T: AsRef<str>>(&self, string: T, max_width: usize, align: Align) -> String {
        clip_str(self, string, max_width, align)
    }

    /// Alias to [Clip::calc] with [Align::End]
    pub fn calc_end<T: AsRef<str>>(&self, string: T, max_width: usize) -> String {
        self.calc(string, max_width, Align::End)
    }
    /// Alias to [Clip::calc] with [Align::Start]
    pub fn calc_start<T: AsRef<str>>(&self, string: T, max_width: usize) -> String {
        self.calc(string, max_width, Align::Start)
    }
    /// Alias to [Clip::calc] with [Align::Center]
    pub fn calc_center<T: AsRef<str>>(&self, string: T, max_width: usize) -> String {
        self.calc(string, max_width, Align::Center)
    }

    /// Get clip end string
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// assert_eq!(Clip::None.to_string(), None);
    /// assert_eq!(Clip::Clip.to_string(), None);
    /// assert_eq!(Clip::Ellipsis.to_string(), Some("...".into()));
    /// assert_eq!(Clip::Custom(">".into()).to_string(), Some(">".into()));
    /// ```
    pub fn to_string(&self) -> Option<CompactString> {
        match self {
            Self::Ellipsis => Some("...".into()),
            Self::Custom(s) => Some(s.clone()),
            _ => None
        }
    }
}
impl<T: Into<CompactString>> From<T> for Clip {
    fn from(value: T) -> Self {
        Self::Custom(value.into())
    }
}

fn clip_str<T: AsRef<str>>(clip: &Clip, string: T, max_width: usize, align: Align) -> String {
    let string = string.as_ref();
    let str_width = string.width();

    // Do nothing if there is no overflow or if the clip is None
    if str_width <= max_width || clip.eq(&Clip::None) {
        return string.to_string();
    }
    // Return an empty string if an overflow has occured and clip is Hide
    if clip.eq(&Clip::Hide) {
        return String::new();
    }

    let end_str = clip.to_string().unwrap_or(CompactString::default());
    let end_width = end_str.width();
    let clipped_width = max_width.saturating_sub(end_width);

    let result = match align {
        Align::Start => format!(
            "{end_str}{}",
            string.slice(str_width.saturating_sub(clipped_width)..)
        ),
        Align::End => format!(
            "{}{end_str}",
            string.slice(..clipped_width)
        ),
        Align::Center => {
            let (left_pad, right_pad) = if max_width % 2 == 0 {
                (max_width/2, max_width/2)
            } else {
                (max_width/2+1, max_width/2)
            };
            format!(
                "{}{}",
                string.slice(..left_pad),
                string.slice(str_width.saturating_sub(right_pad)..)
            ).place_aligned(end_str, Align::Center)
        }
    };

    if end_width == 0 {
        result
    } else {
        result.slice(..max_width).to_string()
    }
}
