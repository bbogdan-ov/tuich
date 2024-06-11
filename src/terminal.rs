use crate::{backend::{BackendAltScreen, BackendClassicMode, BackendMouse, BackendRawMode}, layout::Rect};
#[allow(unused_imports)]
use crate::{backend::Backend, buffer::{Buffer, Cell}};

#[cfg(feature="backend-event")]
use crate::backend::BackendEvent;
#[cfg(feature="backend-event")]
use crate::event::Event;

/// Terminal
#[derive(Debug, Clone)]
pub struct Terminal<B: Backend> {
    pub backend: B,
    pub buffer: Buffer
}
impl<B: Backend> Terminal<B> {
    /// Create a blank [Terminal]
    pub fn new(backend: B, buffer: Buffer) -> Self {
        Self { backend, buffer }
    }
    /// Create a [Terminal] with a [Buffer] filled with an empty [Cell]
    pub fn empty(backend: B) -> Result<Self, B::Error> {
        let (width, height) = B::terminal_size()?;
        Ok(Self::new(backend, Buffer::empty(width, height)))
    }

    //

    /// Draw terminal buffer
    pub fn draw(&mut self) -> Result<(), B::Error> {
        self.backend.write_buffer(&self.buffer)
    }
    /// Resize the buffer and clear the terminal screen
    pub fn resize(&mut self, width: u16, height: u16) -> Result<(), B::Error> {
        self.backend.clear()?;
        self.buffer.resize(width, height);

        Ok(())
    }
    /// Clear the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Get size of the terminal buffer
    pub fn size(&self) -> (u16, u16) {
        self.buffer.size()
    }
    /// Get rect of the terminal buffer
    pub fn rect(&self) -> Rect {
        self.buffer.rect()
    }
    /// Get terminal size
    pub fn term_size(&self) -> Result<(u16, u16), B::Error> {
        B::terminal_size()
    }
}
impl<B: BackendAltScreen> Terminal<B> {
    pub fn enter_alt_screen(&mut self) -> Result<(), B::Error> {
        self.backend.enter_alt_screen()
    }
    pub fn leave_alt_screen(&mut self) -> Result<(), B::Error> {
        self.backend.leave_alt_screen()
    }
}
impl<B: BackendRawMode> Terminal<B> {
    pub fn enter_raw_mode(&mut self) -> Result<(), B::Error> {
        self.backend.enter_raw_mode()
    }
    pub fn leave_raw_mode(&mut self) -> Result<(), B::Error> {
        self.backend.leave_raw_mode()
    }
}
impl<B: BackendMouse> Terminal<B> {
    pub fn enable_mouse(&mut self) -> Result<(), B::Error> {
        self.backend.enable_mouse()
    }
    pub fn disable_mouse(&mut self) -> Result<(), B::Error> {
        self.backend.disable_mouse()
    }
}
impl<B: BackendClassicMode> Terminal<B> {
    /// Create [Terminal] and enter *"classic"* mode!
    /// Useful for most *classic* tui apps
    pub fn classic(mut backend: B) -> Result<Self, B::Error> {
        backend.enter_classic_mode()?;
        Self::empty(backend)
    }

    pub fn enter_classic_mode(&mut self) -> Result<(), B::Error> {
        self.backend.enter_classic_mode()
    }
}

#[cfg(feature="backend-event")]
impl<B: BackendEvent + Backend> BackendEvent for Terminal<B> {
    type EventError = B::EventError;

    fn read_events(&mut self) -> Result<Event, Self::EventError> {
        self.backend.read_events()
    }
}

impl<B: Backend> Drop for Terminal<B> {
    fn drop(&mut self) {
        let _ = self.backend.reset();
    }
}
