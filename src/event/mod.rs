mod key;
mod mouse;

pub use key::*;
pub use mouse::*;

#[allow(unused)]
use crate::backend::crossterm::CrosstermBackend;

/// Event
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Event {
    Key(Key, KeyCode),
    Mouse(Mouse, u16, u16),
    Paste(String),
    Resize(u16, u16),
    /// Focus gained
    Focus,
    /// Focus lost
    Blur,
    Unknown
}

/// Key modifier
#[derive(Debug, Default, Clone, Copy, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum KeyMod {
    #[default]
    #[cfg_attr(feature="serde", serde(alias=""))]
    Any,
    None,
    Shift,
    Ctrl,
    Alt,
    ShiftCtrl,
    ShiftAlt,
    ShiftCtrlAlt,
    CtrlAlt
}
impl KeyMod {
    /// Returns tuple of booleans `(shift, ctrl, alt)`
    pub fn tuple(&self) -> (bool, bool, bool) {
        match self {
            Self::Shift        => (true,  false, false),
            Self::Ctrl         => (false, true,  false),
            Self::Alt          => (false, false, true),
            Self::ShiftCtrl    => (true,  true,  false),
            Self::ShiftAlt     => (true,  false, true),
            Self::ShiftCtrlAlt => (true,  true,  true),
            Self::CtrlAlt      => (false, true,  true),
            Self::None         => (false, false, false),
            Self::Any          => (false, false, false),
        }
    }
}
impl PartialEq for KeyMod {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// Will always return `true` if `self` or `other` are equal to [KeyMod::Any]
    fn eq(&self, other: &Self) -> bool {
        match other {
            Self::Any => true,
            _ => match self {
                Self::Any          => true,
                Self::Shift        => match other { Self::Shift => true, _ => false },
                Self::Ctrl         => match other { Self::Ctrl => true, _ => false },
                Self::Alt          => match other { Self::Alt => true, _ => false },
                Self::ShiftCtrl    => match other { Self::ShiftCtrl => true, _ => false },
                Self::ShiftAlt     => match other { Self::ShiftAlt => true, _ => false },
                Self::ShiftCtrlAlt => match other { Self::ShiftCtrlAlt => true, _ => false },
                Self::CtrlAlt      => match other { Self::CtrlAlt => true, _ => false },
                Self::None         => match other { Self::None => true, _ => false },
            }
        }
    }
}
impl From<(bool, bool, bool)> for KeyMod {
    fn from(value: (bool, bool, bool)) -> Self {
        match value {
            (true,  false, false) => Self::Shift,
            (false, true,  false) => Self::Ctrl,
            (false, false, true)  => Self::Alt,
            (true,  true,  false) => Self::ShiftCtrl,
            (true,  false, true)  => Self::ShiftAlt,
            (true,  true,  true)  => Self::ShiftCtrlAlt,
            (false, true,  true)  => Self::CtrlAlt,
            (false, false, false) => Self::None
        }
    }
}
impl Into<(bool, bool, bool)> for KeyMod {
    fn into(self) -> (bool, bool, bool) {
        self.tuple()
    }
}
