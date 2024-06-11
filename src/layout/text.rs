use stringslice::StringSlice;
use unicode_width::UnicodeWidthStr;

use super::Align;

/// Place
pub trait Place {
    /// Place `s` into some position
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let str = "place😭 some other string here!";
    /// let s = "|place me!|";
    ///
    /// assert_eq!(str.place(s, 0),  "|place me!| other string here!");
    /// assert_eq!(str.place(s, 2),  "pl|place me!|ther string here!");
    /// assert_eq!(str.place(s, 24), "place😭 some other string|place me!|");
    /// ```
    fn place<S: AsRef<str>>(&self, s: S, pos: usize) -> String;
    /// Align `s` inside this string
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let str = "place😭 some other string here";
    /// let s = "|place me!|";
    ///
    /// assert_eq!(str.place_aligned(s, Align::Start),  "|place me!| other string here");
    /// assert_eq!(str.place_aligned(s, Align::End),    "place😭 some other |place me!|");
    /// assert_eq!(str.place_aligned(s, Align::Center), "place😭 som|place me!|ing here");
    /// ```
    fn place_aligned<S: AsRef<str>>(&self, s: S, align: Align) -> String;
}
impl<T: AsRef<str>> Place for T {
    fn place<S: AsRef<str>>(&self, s: S, pos: usize) -> String {
        let str = self.as_ref();
        let str_width = str.width();
        let s = s.as_ref();
        let s_width = s.width();

        if pos >= str_width {
            return format!("{}{s}", str.to_string());
        }
        
        format!(
            "{}{s}{}",
            str.slice(..pos),
            str.slice(pos + s_width..),
        )
    }
    fn place_aligned<S: AsRef<str>>(&self, s: S, align: Align) -> String {
        let str = self.as_ref();
        let str_width = str.width();
        let s = s.as_ref();
        let s_width = s.width();

        match align {
            Align::Start => format!("{s}{}", str.slice(s_width..)),
            Align::End => format!("{}{s}", str.slice(..str_width.saturating_sub(s_width+1))),
            Align::Center => {
                let s_pos = align.calc(s_width, str_width);
                format!(
                    "{}{s}{}",
                    str.slice(..s_pos),
                    str.slice(s_pos + s_width..)
                )
            }
        }
    }
}
