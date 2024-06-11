use super::KeyMod;

/// Mouse button
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum MouseBtn {
    #[default]
    Left,
    Middle,
    Right
}

/// Mouse event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Mouse {
    /// Press
    Down(KeyMod, MouseBtn, u16, u16),
    /// Release
    Up(KeyMod, MouseBtn, u16, u16),
    /// Press and move
    Drag(KeyMod, MouseBtn, u16, u16),
    /// Just move
    Move(KeyMod, u16, u16),
    ScrollUp(KeyMod, u16, u16),
    ScrollDown(KeyMod, u16, u16),
    ScrollLeft(KeyMod, u16, u16),
    ScrollRight(KeyMod, u16, u16),
    Unknown
}
impl Mouse {
    /// Returns mouse button if exists
    pub fn btn(self) -> Option<MouseBtn> {
        match self {
            Self::Down(_, b, _, _) => Some(b),
            Self::Up(_, b, _, _) => Some(b),
            Self::Drag(_, b, _, _) => Some(b),
            Self::Move(_, _, _) => None,
            Self::ScrollUp(_, _, _) => None,
            Self::ScrollDown(_, _, _) => None,
            Self::ScrollLeft(_, _, _) => None,
            Self::ScrollRight(_, _, _) => None,
            Self::Unknown => None
        }
    }
    /// Returns key modifier if exists
    pub fn key_mod(self) -> Option<KeyMod> {
        match self {
            Self::Down(m, _, _, _) => Some(m),
            Self::Up(m, _, _, _) => Some(m),
            Self::Drag(m, _, _, _) => Some(m),
            Self::Move(m, _, _) => Some(m),
            Self::ScrollUp(m, _, _) => Some(m),
            Self::ScrollDown(m, _, _) => Some(m),
            Self::ScrollLeft(m, _, _) => Some(m),
            Self::ScrollRight(m, _, _) => Some(m),
            Self::Unknown => None
        }
    }
    /// Returns mouse position if exists
    pub fn pos(self) -> Option<(u16, u16)> {
        match self {
            Self::Down(_, _, x, y) => Some((x, y)),
            Self::Up(_, _, x, y) => Some((x, y)),
            Self::Drag(_, _, x, y) => Some((x, y)),
            Self::Move(_, x, y) => Some((x, y)),
            Self::ScrollUp(_, x, y) => Some((x, y)),
            Self::ScrollDown(_, x, y) => Some((x, y)),
            Self::ScrollLeft(_, x, y) => Some((x, y)),
            Self::ScrollRight(_, x, y) => Some((x, y)),
            Self::Unknown => None
        }
    }
}
