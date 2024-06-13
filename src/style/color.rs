use super::{Style, Stylized};

/// Color
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Color {
    #[default]
    /// Reset color to the terminal's default color
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    #[cfg_attr(feature="serde", serde(untagged, deserialize_with="deserialize_rgb"))]
    Rgb(u8, u8, u8),
    #[cfg_attr(feature="serde", serde(untagged))]
    Ansi(u8),
}
impl From<()> for Color {
    /// Creates a default [Color]
    fn from(_: ()) -> Self {
        Self::default()
    }
}
impl From<u8> for Color {
    /// Creates [Color::Ansi] from the ansi value
    fn from(value: u8) -> Self {
        Self::Ansi(value)
    }
}
impl From<(u8, u8, u8)> for Color {
    /// Creates [Color::Rgb] from the RGB tuple `(red, green, blue)`
    fn from(v: (u8, u8, u8)) -> Self {
        Self::Rgb(v.0, v.1, v.2)
    }
}

#[cfg(feature="text-stylized")]
impl Stylized for Color {
    type Output = Style;

    fn style<S: Into<Style>>(self, style: S) -> Self::Output {
        style.into()
    }
    fn get_style(&self) -> Style {
        Style::from(*self)
    }
}

// Deserialization
#[cfg(feature="serde")]
struct ColorRgbVisitor;
#[cfg(feature="serde")]
impl<'de> serde::de::Visitor<'de> for ColorRgbVisitor {
    type Value = (u8, u8, u8);

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[red, green, blue] or #RRGGBB")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
    {
        if let Some(hex) = v.strip_prefix("#") {
            // Thanks! https://github.com/crossterm-rs/crossterm/blob/master/src/style/types/color.rs#L314
            // Trying to parse HEX
            if hex.is_ascii() && hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16);
                let g = u8::from_str_radix(&hex[2..4], 16);
                let b = u8::from_str_radix(&hex[4..6], 16);

                if r.is_ok() && g.is_ok() && b.is_ok() {
                    return Ok((
                        r.unwrap(),
                        g.unwrap(),
                        b.unwrap(),
                    ));
                }
            }
        }

        Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where A: serde::de::SeqAccess<'de>
    {
        let r = seq.next_element::<u8>()?
            .ok_or(serde::de::Error::missing_field("red"))?;
        let g = seq.next_element::<u8>()?
            .ok_or(serde::de::Error::missing_field("green"))?;
        let b = seq.next_element::<u8>()?
            .ok_or(serde::de::Error::missing_field("blue"))?;

        if let Some(v) = seq.next_element::<u8>()? {
            return Err(serde::de::Error::custom(format!("unexpected additional value {} in RGB sequence", v)));
        }

        Ok((r, g, b))
    }
}

#[cfg(feature="serde")]
fn deserialize_rgb<'de, D>(des: D) -> Result<(u8, u8, u8), D::Error>
where D: serde::Deserializer<'de>
{
    des.deserialize_any(ColorRgbVisitor)
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_color() {
        assert_eq!(Color::from(()), Color::default());
        assert_eq!(Color::from((10, 20, 30)), Color::Rgb(10, 20, 30));
        assert_eq!(Color::from(10), Color::Ansi(10));
    }
}

#[cfg(test)]
#[cfg(feature="serde")]
mod serde_tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct C { c: Color }

    fn color(s: &str) -> Result<Color, toml::de::Error> {
        let tbl = toml::from_str::<C>(s)?;
        Ok(tbl.c)
    }

    #[test]
    fn color_des() {
        assert_eq!(color("c = \"reset\"").unwrap(),     Color::Reset);
        assert_eq!(color("c = \"red\"").unwrap(),       Color::Red);
        assert_eq!(color("c = \"light_red\"").unwrap(), Color::LightRed);
        assert_eq!(color("c = 10").unwrap(),            Color::Ansi(10));
        assert_eq!(color("c = [10, 255, 0]").unwrap(),  Color::Rgb(10, 255, 0));
        assert_eq!(color("c = \"#ff0000\"").unwrap(),   Color::Rgb(255, 0, 0));
        assert_eq!(color("c = \"#00ffff\"").unwrap(),   Color::Rgb(0, 255, 255));

        assert!(color("c = \"Reset\"").is_err());
        assert!(color("c = \"lightred\"").is_err());
        assert!(color("c = \"light-red\"").is_err());
        assert!(color("c = \"hey\"").is_err());
        assert!(color("c = 256").is_err());
        assert!(color("c = [256, 255, 0]").is_err());
        assert!(color("c = [1, 10]").is_err());
        assert!(color("c = \"#f00\"").is_err());
        assert!(color("c = \"#ff00f\"").is_err());
    }
    #[test]
    fn color_ser() {
        assert_eq!(
            toml::to_string(&C { c: Color::Red }).unwrap().trim(),
            "c = \"red\""
        );
        assert_eq!(
            toml::to_string(&C { c: Color::LightRed }).unwrap().trim(),
            "c = \"light_red\""
        );
        assert_eq!(
            toml::to_string(&C { c: Color::Rgb(10, 20, 200) }).unwrap().trim(),
            "c = [10, 20, 200]"
        );
        assert_eq!(
            toml::to_string(&C { c: Color::Ansi(10) }).unwrap().trim(),
            "c = 10"
        );
    }
}
