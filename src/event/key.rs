use super::KeyMod;

/// Modifier key code
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum ModKeyCode {
    Shift,
    Ctrl,
    Alt,
    /// Meta/win/super key
    Super,
    Unknown
}

/// Key code
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum KeyCode {
    Backspace,
    Enter,
    Tab,
    BackTab,
    Delete,

    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,

    Insert,
    Esc,

    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    CapsLock,
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ScrollLock,
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    NumLock,
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    PrintScreen,
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Pause,
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    Menu,
    /// The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
    ///
    /// **For [CrosstermBackend]:** this key can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    KeypadBegin,

    #[default]
    #[cfg_attr(feature="serde", serde(alias=""))]
    None,
    Unknown,

    #[cfg_attr(feature="serde", serde(untagged, deserialize_with="deserialize_char_key_code"))]
    Char(char),
    /// TODO: create custom deserializer/serializer
    #[cfg_attr(feature="serde", serde(untagged, deserialize_with="deserialize_f_key_code"))]
    F(u8),
    /// A modifier key
    ///
    /// **For [CrosstermBackend]:** these keys can only be read if **both**
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] and
    /// [`KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES`] have been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    #[cfg_attr(feature="serde", serde(untagged))]
    Mod(ModKeyCode),
}

/// Key event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key(pub KeyMod, pub KeyCode);

impl Key {
    /// Returns a key modifiers
    pub fn key_mod(self) -> KeyMod {
        self.0
    }
    /// Returns a key code
    pub fn code(self) -> KeyCode {
        self.1
    }
}
impl<M, C> From<(M, C)> for Key
where M: Into<KeyMod>,
      C: Into<KeyCode>
{
    fn from(v: (M, C)) -> Self {
        Self(v.0.into(), v.1.into())
    }
}

#[cfg(feature="serde")]
struct KeyCodeFVisitor;
#[cfg(feature="serde")]
impl<'de> serde::de::Visitor<'de> for KeyCodeFVisitor {
    type Value = u8;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "f1, f2, ..., f12")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        if let Some(n) = v.strip_prefix("f") {
            let Ok(n) = n.parse::<u8>() else {
                return Err(E::custom(format!("expected f1, f2, ..., f12, but {} was received", v)));
            };
            if n > 12 {
                return Err(E::custom(format!("expected f1, f2, ..., f12, but {} was received", v)));
            }

            return Ok(n)
        }

        Err(E::custom(format!("expected f1, f2, ..., f12, but {} was received", v)))
    }
}

#[cfg(feature="serde")]
struct KeyCodeCharVisitor;
#[cfg(feature="serde")]
impl<'de> serde::de::Visitor<'de> for KeyCodeCharVisitor {
    type Value = char;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "character or number 0-9")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        let mut chars = v.chars();
        let Some(char) = chars.next() else {
            return Err(E::custom("expected a character"));
        };
        if chars.next().is_some() {
            return Err(E::custom(format!("expected only one character, got {}", v)));
        }

        Ok(char)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        if v < 0 || v > 9 {
            return Err(E::custom(format!("unexpected number character {}, must be 0-9", v)))
        }

        Ok(v.to_string().chars().nth(0).unwrap())
    }
}

#[cfg(feature="serde")]
fn deserialize_f_key_code<'de, D>(des: D) -> Result<u8, D::Error>
where D: serde::Deserializer<'de>
{
    des.deserialize_str(KeyCodeFVisitor)
}
#[cfg(feature="serde")]
fn deserialize_char_key_code<'de, D>(des: D) -> Result<char, D::Error>
where D: serde::Deserializer<'de>
{
    des.deserialize_any(KeyCodeCharVisitor)
}



// Tests
#[cfg(feature="serde")]
#[cfg(test)]
mod serde_tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct K {
        k: Key
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    struct C {
        c: KeyCode
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    struct M {
        m: ModKeyCode
    }

    fn key(s: &str) -> Result<Key, toml::de::Error> {
        let tbl = toml::from_str::<K>(s)?;
        Ok(tbl.k)
    }
    fn key_code(s: &str) -> Result<KeyCode, toml::de::Error> {
        let tbl = toml::from_str::<C>(s)?;
        Ok(tbl.c)
    }
    fn mod_key_code(s: &str) -> Result<ModKeyCode, toml::de::Error> {
        let tbl = toml::from_str::<M>(s)?;
        Ok(tbl.m)
    }

    #[test]
    fn des_key() {
        assert_eq!(key(r#"k = ["", "none"]"#).unwrap(), Key(KeyMod::None, KeyCode::None));
        assert_eq!(key(r#"k = ["", "a"]"#).unwrap(), Key(KeyMod::None, KeyCode::Char('a')));
        assert_eq!(key(r#"k = ["", "f2"]"#).unwrap(), Key(KeyMod::None, KeyCode::F(2)));
        assert_eq!(key(r#"k = ["shift", "b"]"#).unwrap(), Key(KeyMod::Shift, KeyCode::Char('b')));
        assert_eq!(key(r#"k = ["shift_ctrl", "shift"]"#).unwrap(), Key(KeyMod::ShiftCtrl, KeyCode::Mod(ModKeyCode::Shift)));

        assert!(key(r#"k = "a""#).is_err());
        assert!(key(r#"k = "shift""#).is_err());
        assert!(key(r#"k = []"#).is_err());
        assert!(key(r#"k = ["foo", "a"]"#).is_err());
        assert!(key(r#"k = ["bar"]"#).is_err());
    }

    #[test]
    fn des_key_code() {
        assert_eq!(key_code(r#"c = "a""#).unwrap(), KeyCode::Char('a'));
        assert_eq!(key_code(r#"c = "A""#).unwrap(), KeyCode::Char('A'));
        assert_eq!(key_code(r#"c = 1"#).unwrap(), KeyCode::Char('1'));
        assert_eq!(key_code(r#"c = "shift""#).unwrap(), KeyCode::Mod(ModKeyCode::Shift));
        assert_eq!(key_code(r#"c = "f1""#).unwrap(), KeyCode::F(1));
        assert_eq!(key_code(r#"c = "f12""#).unwrap(), KeyCode::F(12));
        assert_eq!(key_code(r#"c = "none""#).unwrap(), KeyCode::None);
        assert_eq!(key_code(r#"c = """#).unwrap(), KeyCode::None);

        assert!(key_code(r#"c = "ab""#).is_err());
        assert!(key_code(r#"c = 10"#).is_err());
        assert!(key_code(r#"c = "f13""#).is_err());
        assert!(key_code(r#"c = "shift_ctrl""#).is_err());
    }
    
    #[test]
    fn des_mod_key_code() {
        assert_eq!(mod_key_code(r#"m = "shift""#).unwrap(), ModKeyCode::Shift);
        assert_eq!(mod_key_code(r#"m = "ctrl""#).unwrap(), ModKeyCode::Ctrl);
        assert_eq!(mod_key_code(r#"m = "alt""#).unwrap(), ModKeyCode::Alt);
        assert_eq!(mod_key_code(r#"m = "super""#).unwrap(), ModKeyCode::Super);

        assert!(mod_key_code(r#"m = """#).is_err());
        assert!(mod_key_code(r#"m = 1"#).is_err());
        assert!(mod_key_code(r#"m = "foo""#).is_err());
    }
}
