#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

use crate::layout::Side;

use super::glyphs::{DOUBLE_BORDER, ROUNDED_BORDER, SINGLE_BORDER, THICK_BORDER, BLOCK_BORDER};

/// Border kind
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize), serde(rename_all="snake_case"))]
pub enum BorderKind {
    #[default]
    Single,
    Double,
    Rounded,
    Thick,
    Block,
    /// Custom border characters `[left, top-left, top, top-right, right, bottom-right, bottom, bottom-left]`
    #[cfg_attr(feature="serde", serde(untagged))]
    Custom([char; 8])
}
impl BorderKind {
    pub fn chars(&self) -> &[char; 8] {
        match self {
            Self::Single => &SINGLE_BORDER,
            Self::Double => &DOUBLE_BORDER,
            Self::Rounded => &ROUNDED_BORDER,
            Self::Thick => &THICK_BORDER,
            Self::Block => &BLOCK_BORDER,
            Self::Custom(chars) => chars,
        }
    }
    /// Get char in some side
    /// 
    /// # Examples
    ///
    /// ```
    /// # use tuich::{style::*, layout::*};
    /// let single = BorderKind::Single;
    /// let double = BorderKind::Double;
    /// let custom = BorderKind::Custom(['|', '+', '=', '+', '|', '*', '=', '*']);
    ///
    /// assert_eq!(single.char_at(Side::TopLeft), '┌');
    /// assert_eq!(double.char_at(Side::Bottom), '═');
    /// assert_eq!(custom.char_at(Side::BottomLeft), '*');
    /// ```
    pub fn char_at(&self, side: Side) -> char {
        let index: usize = side.into();
        self.chars()[index]
    }
    /// Same as [BorderKind::char_at], but returns [String]
    pub fn string_at(&self, side: Side) -> String {
        self.char_at(side).to_string()
    }
}

// Tests
#[cfg(test)]
#[cfg(feature="serde")]
mod serde_tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct B { b: BorderKind }

    fn kind(s: &str) -> Result<BorderKind, toml::de::Error> {
        let tbl = toml::from_str::<B>(s)?;
        Ok(tbl.b)
    }

    #[test]
    fn border_kind_des() {
        assert_eq!(kind(r#"b = "single""#).unwrap(), BorderKind::Single);
        assert_eq!(
            kind(r#"b = ["a", "b", "c", "d", "e", "f", "g", "h"]"#).unwrap(),
            BorderKind::Custom(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'])
        );

        assert!(kind(r#"b = "foo""#).is_err());
    }
}
