use crate::{buffer::Buffer, style::Style};

#[cfg(feature="backend-event")]
use crate::event::{Event, Key, Mouse};

#[cfg(feature="backend-event")]
pub trait BackendEventReader {
    type EventError;

    fn read_events(&mut self) -> Result<Event, Self::EventError>;
    fn read_keys(&mut self) -> Result<Option<Key>, Self::EventError> {
        let event = self.read_events()?;

        if let Event::Key(key, _) = event {
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }
    fn read_mouse(&mut self) -> Result<Option<Mouse>, Self::EventError> {
        let event = self.read_events()?;

        if let Event::Mouse(mouse, _, _) = event {
            Ok(Some(mouse))
        } else {
            Ok(None)
        }
    }
}

/// Backend
/// Base trait that every backend must have
pub trait Backend {
    type Error;

    /// Write to the terminal output
    fn write<S: AsRef<str>>(&mut self, s: S) -> Result<(), Self::Error>;
    /// Flush the terminal screen
    fn flush(&mut self) -> Result<(), Self::Error>;
    /// Clear the terminal screen
    fn clear(&mut self) -> Result<(), Self::Error>;
    fn show_cursor(&mut self) -> Result<(), Self::Error>;
    fn hide_cursor(&mut self) -> Result<(), Self::Error>;
    fn place_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error>;
    /// Write a style to the terminal output
    fn write_style(&mut self, style: &Style, last_style: &Style) -> Result<(), Self::Error>;
    /// Reset all applied styles
    fn write_reset(&mut self) -> Result<(), Self::Error>;

    /// Write a buffer to the terminal output
    fn write_buffer(&mut self, buffer: &Buffer) -> Result<(), Self::Error> {
        self.place_cursor(0, 0)?;
        self.write_style(&Style::default(), &Style::default())?;

        let mut last_style = &Style::default();
        let mut skip_next = false;

        for (index, cell) in buffer.cells.iter().enumerate() {
            let (x, y) = buffer.pos_of(index).unwrap();

            if skip_next {
                skip_next = false;
                continue;
            }
            if cell.display_width() == 2 {
                skip_next = true;
            }

            if cell.style.ne(&last_style) {
                self.write_style(&cell.style, &last_style)?;
                last_style = &cell.style;
            }

            if let Some(char) = &cell.char {
                self.write(char)?;
            } else {
                self.write(" ")?;
            }

            if x >= buffer.width.saturating_sub(1) {
                self.place_cursor(0, y + 1)?;
            }
        }

        self.write_reset()?;
        self.flush()
    }

    /// Reset terminal state to default
    fn reset(&mut self) -> Result<(), Self::Error>;

    fn terminal_size() -> Result<(u16, u16), Self::Error>;
}

/// Backend alternate screen
/// A backend with ability to enter alternate screen
pub trait BackendAltScreen: Backend {
    fn enter_alt_screen(&mut self) -> Result<(), Self::Error>;
    fn leave_alt_screen(&mut self) -> Result<(), Self::Error>;
}
/// Backend raw mode
/// A backend with ability to enable raw mode
pub trait BackendRawMode: Backend {
    fn enter_raw_mode(&mut self) -> Result<(), Self::Error>;
    fn leave_raw_mode(&mut self) -> Result<(), Self::Error>;
}
/// Backend mouse
/// A backend with mouse support
pub trait BackendMouse: Backend {
    fn enable_mouse(&mut self) -> Result<(), Self::Error>;
    fn disable_mouse(&mut self) -> Result<(), Self::Error>;
}

pub trait BackendClassicMode: Backend {
    /// Enter raw mode, enter alternate screen and hide the cursor
    fn enter_classic_mode(&mut self) -> Result<(), Self::Error>;
}

#[cfg(feature="backend-event")]
pub trait BackendEvent {
    type EventReader: BackendEventReader;
    fn event_reader(&self) -> Self::EventReader;
}

impl<T: BackendAltScreen + BackendRawMode> BackendClassicMode for T {
    fn enter_classic_mode(&mut self) -> Result<(), Self::Error> {
        self.enter_raw_mode()?;
        self.enter_alt_screen()?;
        self.hide_cursor()?;
        self.place_cursor(0, 0)
    }
}
