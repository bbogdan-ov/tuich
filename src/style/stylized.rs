#[cfg(feature="text-span")]
use std::borrow::Cow;

#[cfg(feature="text-span")]
use crate::text::Span;

use super::{Color, Style, UnderlineKind};

/// Stylized
pub trait Stylized: Sized {
    type Output;

    /// Set style
    fn style<S: Into<Style>>(self, style: S) -> Self::Output;
    /// Get style
    fn get_style(&self) -> Style;

    /// Set foreground color
    fn fg<C: Into<Color>, O: Into<Option<C>>>(self, color: O) -> Self::Output {
        let style = self.get_style().fg(color);
        self.style(style)
    }
    /// Set background color
    fn bg<C: Into<Color>, O: Into<Option<C>>>(self, color: O) -> Self::Output {
        let style = self.get_style().bg(color);
        self.style(style)
    }

    /// Reset foreground
    fn fg_reset(self) -> Self::Output { self.fg(Color::Reset) }
    /// Reset background
    fn bg_reset(self) -> Self::Output { self.bg(Color::Reset) }

    /// Set black foreground
    fn black(self) -> Self::Output { self.fg(Color::Black) }
    /// Set red foreground
    fn red(self) -> Self::Output { self.fg(Color::Red) }
    /// Set green foreground
    fn green(self) -> Self::Output { self.fg(Color::Green) }
    /// Set yellow foreground
    fn yellow(self) -> Self::Output { self.fg(Color::Yellow) }
    /// Set blue foreground
    fn blue(self) -> Self::Output { self.fg(Color::Blue) }
    /// Set magenta foreground
    fn magenta(self) -> Self::Output { self.fg(Color::Magenta) }
    /// Set cyan foreground
    fn cyan(self) -> Self::Output { self.fg(Color::Cyan) }
    /// Set gray foreground
    fn gray(self) -> Self::Output { self.fg(Color::Gray) }
    /// Set light black foreground
    fn light_black(self) -> Self::Output { self.fg(Color::LightBlack) }
    /// Set light red foreground
    fn light_red(self) -> Self::Output { self.fg(Color::LightRed) }
    /// Set light green foreground
    fn light_green(self) -> Self::Output { self.fg(Color::LightGreen) }
    /// Set light yellow foreground
    fn light_yellow(self) -> Self::Output { self.fg(Color::LightYellow) }
    /// Set light blue foreground
    fn light_blue(self) -> Self::Output { self.fg(Color::LightBlue) }
    /// Set light magenta foreground
    fn light_magenta(self) -> Self::Output { self.fg(Color::LightMagenta) }
    /// Set light cyan foreground
    fn light_cyan(self) -> Self::Output { self.fg(Color::LightCyan) }
    /// Set light gray foreground
    fn light_gray(self) -> Self::Output { self.fg(Color::LightGray) }

    /// Set black background
    fn on_black(self) -> Self::Output { self.bg(Color::Black) }
    /// Set red background
    fn on_red(self) -> Self::Output { self.bg(Color::Red) }
    /// Set green on_background
    fn on_green(self) -> Self::Output { self.bg(Color::Green) }
    /// Set yellow background
    fn on_yellow(self) -> Self::Output { self.bg(Color::Yellow) }
    /// Set blue background
    fn on_blue(self) -> Self::Output { self.bg(Color::Blue) }
    /// Set magenton_a background
    fn on_magenta(self) -> Self::Output { self.bg(Color::Magenta) }
    /// Set cyan on_background
    fn on_cyan(self) -> Self::Output { self.bg(Color::Cyan) }
    /// Set gray background
    fn on_gray(self) -> Self::Output { self.bg(Color::Gray) }
    /// Set light black background
    fn on_light_black(self) -> Self::Output { self.bg(Color::LightBlack) }
    /// Set light red background
    fn on_light_red(self) -> Self::Output { self.bg(Color::LightRed) }
    /// Set light green on_background
    fn on_light_green(self) -> Self::Output { self.bg(Color::LightGreen) }
    /// Set light yellow background
    fn on_light_yellow(self) -> Self::Output { self.bg(Color::LightYellow) }
    /// Set light blue background
    fn on_light_blue(self) -> Self::Output { self.bg(Color::LightBlue) }
    /// Set light magenton_a background
    fn on_light_magenta(self) -> Self::Output { self.bg(Color::LightMagenta) }
    /// Set light cyan on_background
    fn on_light_cyan(self) -> Self::Output { self.bg(Color::LightCyan) }
    /// Set light gray background
    fn on_light_gray(self) -> Self::Output { self.bg(Color::LightGray) }

    /// Set bold modifier
    fn bold(self) -> Self::Output {
        let style = self.get_style().bold(true);
        self.style(style)
    }
    /// Set italic modifier
    fn italic(self) -> Self::Output {
        let style = self.get_style().italic(true);
        self.style(style)
    }
    /// Set reverse modifier
    fn reverse(self) -> Self::Output {
        let style = self.get_style().reverse(true);
        self.style(style)
    }
    /// Set underline modifier
    fn underline(self) -> Self::Output {
        let style = self.get_style().underline(true);
        self.style(style)
    }
    /// Set underline kind
    fn underline_kind(self, kind: UnderlineKind) -> Self::Output {
        let style = self.get_style().underline_kind(kind);
        self.style(style)
    }
}

#[cfg(feature="text-span")]
impl<'a> Stylized for &'a str {
    type Output = Span<'a>;

    fn style<S: Into<Style>>(self, style: S) -> Self::Output { Span::new(self, style) }
    fn get_style(&self) -> Style { Style::default() }
}
#[cfg(feature="text-span")]
impl<'a> Stylized for Cow<'a, str> {
    type Output = Span<'a>;

    fn style<S: Into<Style>>(self, style: S) -> Self::Output { Span::new(self, style) }
    fn get_style(&self) -> Style { Style::default() }
}
