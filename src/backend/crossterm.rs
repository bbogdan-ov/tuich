//! Backend for the [crossterm](https://docs.rs/crossterm) lib!

use std::io::{self, Write};

use crossterm::{cursor, event::{DisableMouseCapture, EnableMouseCapture}, execute, style::{Attribute, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::{event::{Event, Key, KeyCode, KeyMod, ModKeyCode, Mouse, MouseBtn}, style::{Color, Style, UnderlineKind}};

use super::{Backend, BackendAltScreen, BackendEvent, BackendMouse, BackendRawMode};

/// [Crossterm](https://docs.rs/crossterm) backend
/// 
/// # Implements
///
/// - [BackendAltScreen] - alternate screen support
/// - [BackendRawMode] - raw mode support
/// - [BackendMouse] - mouse support
/// - [BackendEvent] - custom events wrapper
#[derive(Debug, Clone)]
pub struct CrosstermBackend<W: Write>(pub W);
impl<W: Write> Backend for CrosstermBackend<W> {
    type Error = io::Error;

    fn write<S: AsRef<str>>(&mut self, s: S) -> io::Result<()> {
        write!(self, "{}", s.as_ref())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Write::flush(self)
    }
    fn clear(&mut self) -> Result<(), Self::Error> {
        write!(self, "{}", Clear(ClearType::All))
    }
    fn show_cursor(&mut self) -> io::Result<()> {
        write!(self, "{}", cursor::Show)
    }
    fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self, "{}", cursor::Hide)
    }
    fn place_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self, "{}", cursor::MoveTo(x, y))
    }
    fn write_style(&mut self, style: &Style, last_style: &Style) -> io::Result<()> {
        write_style(self, style, last_style)
    }

    fn reset(&mut self) -> Result<(), Self::Error> {
        self.leave_alt_screen()?;
        self.leave_raw_mode()?;
        self.show_cursor()?;
        self.disable_mouse()
    }

    fn terminal_size() -> io::Result<(u16, u16)> {
        crossterm::terminal::size()
    }
}
impl<W: Write> BackendAltScreen for CrosstermBackend<W> {
    fn enter_alt_screen(&mut self) -> io::Result<()> {
        execute!(self.0, EnterAlternateScreen)
    }
    fn leave_alt_screen(&mut self) -> io::Result<()> {
        execute!(self, LeaveAlternateScreen)
    }
}
impl<W: Write> BackendRawMode for CrosstermBackend<W> {
    fn enter_raw_mode(&mut self) -> io::Result<()> {
        enable_raw_mode()
    }
    fn leave_raw_mode(&mut self) -> io::Result<()> {
        disable_raw_mode()
    }
}
impl<W: Write> BackendMouse for CrosstermBackend<W> {
    fn enable_mouse(&mut self) -> Result<(), Self::Error> {
        execute!(self.0, EnableMouseCapture)
    }
    fn disable_mouse(&mut self) -> Result<(), Self::Error> {
        execute!(self.0, DisableMouseCapture)
    }
}

impl<W: Write> Write for CrosstermBackend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
impl Default for CrosstermBackend<io::Stdout> {
    fn default() -> Self {
        CrosstermBackend(io::stdout())
    }
}

#[cfg(feature="backend-crossterm-event")]
impl<W: Write> BackendEvent for CrosstermBackend<W> {
    type EventError = io::Error;

    fn read_events(&mut self) -> Result<Event, Self::EventError> {
        use crossterm::event::Event as E;

        Ok(match crossterm::event::read()? {
            E::Key(key) => {
                let code = KeyCode::from(key.code);
                Event::Key(Key(key.modifiers.into(), code), code)
            },
            E::Mouse(mouse) => Event::Mouse(mouse.into(), mouse.column, mouse.row),
            E::Paste(data) => Event::Paste(data),
            E::Resize(w, h) => Event::Resize(w, h),
            E::FocusGained => Event::Focus,
            E::FocusLost => Event::Blur,
        })
    }
}

// Utils
#[cfg(feature="backend-crossterm-event")]
impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(value: crossterm::event::KeyCode) -> Self {
        use crossterm::event::KeyCode as C;

        match value {
            C::Backspace => Self::Backspace,
            C::Enter => Self::Enter,
            C::Tab => Self::Tab,
            C::BackTab => Self::BackTab,
            C::Delete => Self::Delete,

            C::Left => Self::Left,
            C::Right => Self::Right,
            C::Up => Self::Up,
            C::Down => Self::Down,
            C::Home => Self::Home,
            C::End => Self::End,
            C::PageUp => Self::PageUp,
            C::PageDown => Self::PageDown,

            C::Insert => Self::Insert,
            C::Esc => Self::Esc,

            C::CapsLock => Self::CapsLock,
            C::ScrollLock => Self::ScrollLock,
            C::NumLock => Self::NumLock,
            C::PrintScreen => Self::PrintScreen,
            C::Pause => Self::Pause,
            C::Menu => Self::Menu,
            C::KeypadBegin => Self::KeypadBegin,

            C::Null => Self::None,

            C::Char(char) => Self::Char(char),
            C::F(n) => Self::F(n),
            C::Modifier(code) => Self::Mod(code.into()),

            _ => Self::Unknown,
        }
    }
}

#[cfg(feature="backend-crossterm-event")]
impl From<crossterm::event::ModifierKeyCode> for ModKeyCode {
    fn from(value: crossterm::event::ModifierKeyCode) -> Self {
        use crossterm::event::ModifierKeyCode as C;
        
        match value {
            C::LeftShift | C::RightShift => Self::Shift,
            C::LeftControl | C::RightControl => Self::Ctrl,
            C::LeftAlt | C::RightAlt => Self::Alt,
            C::LeftMeta | C::RightMeta |
            C::LeftHyper | C::RightHyper |
            C::LeftSuper | C::RightSuper => Self::Super,
            _ => Self::Unknown
        }
    }
}

#[cfg(feature="backend-crossterm-event")]
impl From<crossterm::event::MouseEvent> for Mouse {
    fn from(value: crossterm::event::MouseEvent) -> Self {
        use crossterm::event::MouseEventKind as K;

        let x = value.column;
        let y = value.row;
        let mods = KeyMod::from(value.modifiers);

        match value.kind {
            K::Down(btn)   => Self::Down(mods, btn.into(), x, y),
            K::Up(btn)     => Self::Up(mods, btn.into(), x, y),
            K::Drag(btn)   => Self::Drag(mods, btn.into(), x, y),
            K::Moved       => Self::Move(mods, x, y),
            K::ScrollUp    => Self::ScrollUp(mods, x, y),
            K::ScrollDown  => Self::ScrollDown(mods, x, y),
            K::ScrollLeft  => Self::ScrollLeft(mods, x, y),
            K::ScrollRight => Self::ScrollRight(mods, x, y),
        }
    }
}
#[cfg(feature="backend-crossterm-event")]
impl From<crossterm::event::MouseButton> for MouseBtn {
    fn from(value: crossterm::event::MouseButton) -> Self {
        use crossterm::event::MouseButton as B;
        match value {
            B::Left   => Self::Left,
            B::Middle => Self::Middle,
            B::Right  => Self::Right,
        }
    }
}
#[cfg(feature="backend-crossterm-event")]
impl From<crossterm::event::KeyModifiers> for KeyMod {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        use crossterm::event::KeyModifiers as M;

        if value == M::SHIFT { Self::Shift }
        else if value == M::CONTROL { Self::Ctrl }
        else if value == M::ALT { Self::Alt }
        else if value == M::SHIFT | M::CONTROL { Self::ShiftCtrl }
        else if value == M::SHIFT | M::ALT { Self::ShiftAlt }
        else if value == M::SHIFT | M::CONTROL | M::ALT { Self::ShiftCtrlAlt }
        else if value == M::CONTROL | M::ALT { Self::CtrlAlt }
        else { Self::None }
    }
}

pub fn color_to_crossterm(color: Color) -> crossterm::style::Color {
    use crossterm::style::Color as C;

    match color {
        Color::Reset        => C::Reset,
        Color::Black        => C::Black,
        Color::Red          => C::DarkRed,
        Color::Green        => C::DarkGreen,
        Color::Yellow       => C::DarkYellow,
        Color::Blue         => C::DarkBlue,
        Color::Magenta      => C::DarkMagenta,
        Color::Cyan         => C::DarkCyan,
        Color::Gray         => C::Grey,
        Color::LightBlack   => C::DarkGrey,
        Color::LightRed     => C::Red,
        Color::LightGreen   => C::Green,
        Color::LightYellow  => C::Yellow,
        Color::LightBlue    => C::Blue,
        Color::LightMagenta => C::Magenta,
        Color::LightCyan    => C::Cyan,
        Color::LightGray    => C::White,
        Color::Rgb(r, g, b) => C::Rgb { r, g, b },
        Color::Ansi(v)      => C::AnsiValue(v)
    }
}

pub fn write_style(f: &mut impl Write, style: &Style, last_style: &Style) -> io::Result<()> {
    let mut reset_attr = false;

    // Set attributes if any
    if style.underline.is_some_and(|v| v) {
        match style.underline_kind {
            Some(UnderlineKind::Line) => write!(f, "{}", Attribute::Underlined)?,
            Some(UnderlineKind::Curl) => write!(f, "{}", Attribute::Undercurled)?,
            Some(UnderlineKind::Dash) => write!(f, "{}", Attribute::Underdashed)?,
            Some(UnderlineKind::Dot) => write!(f, "{}", Attribute::Underdotted)?,
            None => ()
        }
    } else if last_style.underline.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Reset)?;
        reset_attr = true;
    }

    if style.bold.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Bold)?;
    } else if last_style.bold.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Reset)?;
        reset_attr = true;
    }

    if style.italic.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Italic)?;
    } else if last_style.italic.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Reset)?;
        reset_attr = true;
    }

    if style.reverse.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Reverse)?;
    } else if last_style.reverse.is_some_and(|v| v) {
        write!(f, "{}", Attribute::Reset)?;
        reset_attr = true;
    }

    if reset_attr {
        write!(f, "{}", Attribute::Reset)?;
    }

    // Set foreground and background colors
    write!(
        f,
        "{}{}",
        SetBackgroundColor(color_to_crossterm(style.bg.unwrap_or(Color::Reset))),
        SetForegroundColor(color_to_crossterm(style.fg.unwrap_or(Color::Reset)))
    )
}
