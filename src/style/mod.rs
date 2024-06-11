mod color;
mod style;
mod border;
pub mod glyphs;

pub use color::*;
pub use style::*;
pub use border::*;

#[cfg(feature="text-stylized")]
mod stylized;
#[cfg(feature="text-stylized")]
pub use stylized::*;
