use super::Color;

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature="text-stylized")]
use crate::style::Stylized;

// Consts
/// Every color as foreground
pub const FG_COLORS: &[Style] = &[
    Style { fg: Some(Color::Reset), ..Style::cleared() },
    Style { fg: Some(Color::Black), ..Style::cleared() },
    Style { fg: Some(Color::Red), ..Style::cleared() },
    Style { fg: Some(Color::Green), ..Style::cleared() },
    Style { fg: Some(Color::Yellow), ..Style::cleared() },
    Style { fg: Some(Color::Blue), ..Style::cleared() },
    Style { fg: Some(Color::Magenta), ..Style::cleared() },
    Style { fg: Some(Color::Cyan), ..Style::cleared() },
    Style { fg: Some(Color::Gray), ..Style::cleared() },
    Style { fg: Some(Color::LightBlack), ..Style::cleared() },
    Style { fg: Some(Color::LightRed), ..Style::cleared() },
    Style { fg: Some(Color::LightGreen), ..Style::cleared() },
    Style { fg: Some(Color::LightYellow), ..Style::cleared() },
    Style { fg: Some(Color::LightBlue), ..Style::cleared() },
    Style { fg: Some(Color::LightMagenta), ..Style::cleared() },
    Style { fg: Some(Color::LightCyan), ..Style::cleared() },
    Style { fg: Some(Color::LightGray), ..Style::cleared() },
];
/// Every color as background
pub const BG_COLORS: &[Style] = &[
    Style { fg: Some(Color::Reset), bg: Some(Color::Reset), ..Style::cleared() },
    Style { fg: Some(Color::Reset), bg: Some(Color::Black), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Red), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Green), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Yellow), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Blue), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Magenta), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::Cyan), ..Style::cleared() },
    Style { fg: Some(Color::Reset), bg: Some(Color::Gray), ..Style::cleared() },
    Style { fg: Some(Color::Reset), bg: Some(Color::LightBlack), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightRed), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightGreen), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightYellow), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightBlue), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightMagenta), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightCyan), ..Style::cleared() },
    Style { fg: Some(Color::Black), bg: Some(Color::LightGray), ..Style::cleared() },
];

/// Underline kind
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize), serde(rename_all="snake_case"))]
pub enum UnderlineKind {
    #[default]
    Line,
    Curl,
    Dash,
    Dot
}

/// Style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize), serde(rename_all="snake_case", default))]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub reverse: Option<bool>,
    pub underline: Option<bool>,
    pub underline_kind: Option<UnderlineKind>
}
impl Style {
    pub fn new<F, B>(fg: F, bg: B) -> Self
    where F: Into<Color>,
          B: Into<Color>
    {
        Self {
            fg: Some(fg.into()),
            bg: Some(bg.into()),
            underline_kind: Some(UnderlineKind::Line),
            ..Default::default()
        }
    }
    pub const fn empty() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: None,
            italic: None,
            reverse: None,
            underline: None,
            underline_kind: Some(UnderlineKind::Line),
        }
    }
    pub const fn cleared() -> Self {
        Self {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            bold: Some(false),
            italic: Some(false),
            reverse: Some(false),
            underline: Some(false),
            underline_kind: Some(UnderlineKind::Line),
        }
    }

    //

    pub fn set<S: Into<Style>>(mut self, style: S) -> Self {
        let style: Style = style.into();
        self.fg = style.fg.or_else(|| self.fg);
        self.bg = style.bg.or_else(|| self.bg);
        self.bold = style.bold.or_else(|| self.bold);
        self.italic = style.italic.or_else(|| self.italic);
        self.reverse = style.reverse.or_else(|| self.reverse);
        self.underline = style.underline.or_else(|| self.underline);
        self.underline_kind = style.underline_kind.or_else(|| self.underline_kind);
        self
    }

    pub fn fg<C: Into<Color>, O: Into<Option<C>>>(mut self, color: O) -> Self {
        self.fg = color.into().map(|c| c.into());
        self
    }
    pub fn bg<C: Into<Color>, O: Into<Option<C>>>(mut self, color: O) -> Self {
        self.bg = color.into().map(|c| c.into());
        self
    }
    pub fn bold<B: Into<Option<bool>>>(mut self, value: B) -> Self {
        self.bold = value.into();
        self
    }
    pub fn italic<B: Into<Option<bool>>>(mut self, value: B) -> Self {
        self.italic = value.into();
        self
    }
    pub fn reverse<B: Into<Option<bool>>>(mut self, value: B) -> Self {
        self.reverse = value.into();
        self
    }
    pub fn underline<B: Into<Option<bool>>>(mut self, value: B) -> Self {
        self.underline = value.into();
        self
    }
    pub fn underline_kind<K: Into<Option<UnderlineKind>>>(mut self, kind: K) -> Self {
        self.underline_kind = kind.into();
        self
    }
}
impl Default for Style {
    fn default() -> Self {
        Self::cleared()
    }
}
impl From<()> for Style {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}
impl From<Color> for Style {
    fn from(value: Color) -> Self {
        Self::default().fg(value)
    }
}
impl<F, B> From<(F, B)> for Style
where F: Into<Color>,
      B: Into<Color>
{
    fn from(value: (F, B)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(feature="text-stylized")]
impl Stylized for Style {
    type Output = Self;

    fn style<S: Into<Style>>(self, style: S) -> Self::Output {
        style.into()
    }
    fn get_style(&self) -> Style {
        Self::default()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_style() {
        assert_eq!(Style::from(()), Style::default());
        assert_eq!(Style::from(Color::Red), Style::default().fg(Color::Red));
        assert_eq!(Style::from((Color::Green, ())), Style::new(Color::Green, Color::Reset));
        assert_eq!(Style::from((Color::Green, Color::Blue)), Style::new(Color::Green, Color::Blue));
    }
    #[test]
    fn set_style() {
        let empty = Style::default();
        let fg = Style::default()
            .fg(Color::Gray);
        let attr = Style::default()
            .bg(Color::Reset)
            .bold(true);

        let a = Style::default()
            .fg(Color::Red)
            .bg(Color::Green);
        let b = Style::default()
            .fg(Color::Blue)
            .bold(true);

        assert_eq!(empty.set(a), a);
        assert_eq!(empty.set(b), b);

        assert_eq!(fg.set(a), Style::new(Color::Red, Color::Green));
        assert_eq!(fg.set(b), Style::default().fg(Color::Blue).bold(true));

        assert_eq!(attr.set(a), Style::new(Color::Red, Color::Green).bold(true));
        assert_eq!(attr.set(b), Style::new(Color::Blue, Color::Reset).bold(true));
    }
}

#[cfg(test)]
#[cfg(feature="serde")]
mod serde_tests {
    use super::*;

    fn style(s: &str) -> Result<Style, toml::de::Error> {
        toml::from_str::<Style>(s)
    }

    #[test]
    fn style_des() {
        assert_eq!(
            style(r#"
                fg = "red"
                bg = "green"
            "#).unwrap(),
            Style::new(Color::Red, Color::Green)
        );
        assert_eq!(
            style("
                fg = \"#ff0000\"
                bold = true
            ").unwrap(),
            Style::default()
                .fg(Color::Rgb(255, 0, 0))
                .bold(true)
        );
        assert_eq!(
            style(r#"
                fg = 10
                underline = true
                underline_kind = "dash"
            "#).unwrap(),
            Style::default()
                .fg(Color::Ansi(10))
                .underline(true)
                .underline_kind(UnderlineKind::Dash)
        );

        // TODO: this definitely should drop an error
        assert_eq!(
            style(r#"foo = "bar""#).unwrap(),
            Style::default()
        );

        assert!(style(r#"underline_kind = "foo""#).is_err());
        assert!(style(r#"fg = -10"#).is_err());
        assert!(style(r#"bg = [0, 10]"#).is_err());
        assert!(style(r#"bg = [0, 10, 30, 250]"#).is_err());
        assert!(style(r#"bold = "true""#).is_err());
    }

    #[test]
    fn style_ser() {
        assert_eq!(
            toml::to_string(&Style::new(Color::Red, ())).unwrap().trim(),
            "fg = \"red\"\nbg = \"reset\""
        )
    }
}
