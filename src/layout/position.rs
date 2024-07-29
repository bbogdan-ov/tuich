/// Point trait
pub trait Point<T> {
    fn x(self) -> T;
    fn y(self) -> T;
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, value: T) -> Self;
    fn div(self, value: T) -> Self;
}
impl Point<u16> for (u16, u16) {
    fn x(self) -> u16 { self.0 }
    fn y(self) -> u16 { self.1 }
    fn add(self, rhs: Self) -> Self {
        (self.x().saturating_add(rhs.x()), self.y().saturating_add(rhs.y()))
    }
    fn sub(self, rhs: Self) -> Self {
        (self.x().saturating_sub(rhs.x()), self.y().saturating_sub(rhs.y()))
    }
    fn mul(self, value: u16) -> Self {
        (self.x().saturating_mul(value), self.y().saturating_mul(value))
    }
    fn div(self, value: u16) -> Self {
        (self.x().saturating_div(value), self.y().saturating_div(value))
    }
}

/// Margin `(left, top, right, bottom)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Margin(pub u16, pub u16, pub u16, pub u16);
impl Margin {
    /// Get the `left` side
    pub fn left(&self) -> u16 { self.0 }
    /// Get the `top` side
    pub fn top(&self) -> u16 { self.1 }
    /// Get the `right` side
    pub fn right(&self) -> u16 { self.2 }
    /// Get the `bottom` side
    pub fn bottom(&self) -> u16 { self.3 }
    /// Get horizontal sides of the margin `(left, right)`
    pub fn hor(&self) -> (u16, u16) {
        (self.left(), self.right())
    }
    /// Get vertical sides of the margin `(top, bottom)`
    pub fn ver(&self) -> (u16, u16) {
        (self.top(), self.bottom())
    }
}
impl From<u16> for Margin {
    /// Creates a [Margin] with all sides equal to `v`
    fn from(v: u16) -> Self {
        Self(v, v, v, v)
    }
}
impl From<(u16, u16)> for Margin {
    /// Creates a [Margin] with values for horizontal and vertical sides `(horizontal, vertical)`
    fn from(v: (u16, u16)) -> Self {
        Self(v.0, v.1, v.0, v.1)
    }
}
impl From<(u16, u16, u16, u16)> for Margin {
    /// Creates a [Margin] with a value for each side `(left, top, right, bottom)`
    /// Same as `Margin(left, top, right, bottom)`
    fn from(v: (u16, u16, u16, u16)) -> Self {
        Self(v.0, v.1, v.2, v.3)
    }
}

/// Align
/// Useful when you need to align things
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Align {
    /// Always returns `0` when calculating alignment
    #[default]
    Start,
    Center,
    End
}
impl Align {
    /// Align `target` inside `inside`
    /// Returns position/offset of `target`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let target = 10;
    /// let inside = 24;
    ///
    /// assert_eq!(Align::Start.calc(target, inside), 0); // Always will be 0
    /// assert_eq!(Align::Center.calc(target, inside), 7);
    /// assert_eq!(Align::End.calc(target, inside), 14);
    /// ```
    pub fn calc(&self, target: usize, inside: usize) -> usize {
        if target == 0 || inside == 0 { return 0; }

        match self {
            Self::Start => 0,
            Self::Center => ((inside as f32 / 2.0) - (target as f32 / 2.0)).round() as usize,
            Self::End => inside.saturating_sub(target)
        }
    }
}

/// Axis align
/// Horizontal and vertical alignment
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AxisAlign(pub Align, pub Align);
impl AxisAlign {
    /// Align `target` inside `inside`
    /// Returns position of `target`
    ///
    /// # Examples
    ///
    /// ```
    /// # use tuich::layout::*;
    /// let target = (10, 8);
    /// let inside = (24, 30);
    ///
    /// assert_eq!(AxisAlign(Align::Center, Align::Center).calc(target, inside), (7, 11), "Center, center");
    /// assert_eq!(AxisAlign(Align::Start, Align::Center).calc(target, inside), (0, 11), "Left, center");
    /// assert_eq!(AxisAlign(Align::End, Align::End).calc(target, inside), (14, 22), "Right, bottom");
    /// ```
    pub fn calc(&self, target: (u16, u16), inside: (u16, u16)) -> (u16, u16) {
        (
            self.hor().calc(target.0 as usize, inside.0 as usize) as u16,
            self.ver().calc(target.1 as usize, inside.1 as usize) as u16
        )
    }

    pub fn hor(self) -> Align { self.0 }
    pub fn ver(self) -> Align { self.1 }
}
impl From<Align> for AxisAlign {
    fn from(value: Align) -> Self {
        Self(value, value)
    }
}
impl From<(Align, Align)> for AxisAlign {
    fn from(v: (Align, Align)) -> Self {
        Self(v.0, v.1)
    }
}

/// Direction
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical
}

/// Side
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all="snake_case"))]
pub enum Side {
    #[default]
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft
}
impl From<usize> for Side {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::TopLeft,
            2 => Self::Top,
            3 => Self::TopRight,
            4 => Self::Right,
            5 => Self::BottomRight,
            6 => Self::Bottom,
            7 => Self::BottomLeft,
            _ => Self::Left,
        }
    }
}
impl Into<usize> for Side {
    fn into(self) -> usize {
        match self {
            Self::Left        => 0,
            Self::TopLeft     => 1,
            Self::Top         => 2,
            Self::TopRight    => 3,
            Self::Right       => 4,
            Self::BottomRight => 5,
            Self::Bottom      => 6,
            Self::BottomLeft  => 7,
        }
    }
}

/// Clamp
/// TODO: derive serde serialization and deserialization
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Clamp<T=u16> {
    #[default]
    None,
    Min(T),
    Max(T),
    MinMax(T, T)
}
impl<T> Clamp<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> Self {
        match (min, max) {
            (None, None)       => Self::None,
            (Some(n), None)    => Self::Min(n),
            (None, Some(x))    => Self::Max(x),
            (Some(n), Some(x)) => Self::MinMax(n, x)
        }
    }

    //

    /// Get min value if exists
    pub fn min(self) -> Option<T> {
        match self {
            Self::Min(min) => Some(min),
            Self::MinMax(min, _) => Some(min),
            _ => None
        }
    }
    /// Get max value if exists
    pub fn max(self) -> Option<T> {
        match self {
            Self::Max(max) => Some(max),
            Self::MinMax(_, max) => Some(max),
            _ => None
        }
    }

    pub fn set_min(self, min: T) -> Self {
        match self {
            Self::MinMax(_, max) => Self::MinMax(min, max),
            Self::Max(max) => Self::MinMax(min, max),
            _ => Self::Min(min)
        }
    }
    pub fn set_max(self, max: T) -> Self {
        match self {
            Self::MinMax(min, _) => Self::MinMax(min, max),
            Self::Min(min) => Self::MinMax(min, max),
            _ => Self::Max(max)
        }
    }
}
impl<T: Ord> Clamp<T> {
    /// Clamp `value`
    pub fn calc(self, value: T) -> T {
        match self {
            Self::None => value,
            Self::Min(min) => value.max(min),
            Self::Max(max) => value.min(max),
            Self::MinMax(min, max) => value.clamp(min, max),
        }
    }

    pub fn in_bounds(&self, value: T) -> bool {
        match self {
            Self::None => true,
            Self::Min(min) => value.gt(min),
            Self::Max(max) => value.lt(max),
            Self::MinMax(min, max) => value.gt(min) && value.lt(max)
        }
    }
}
impl From<()> for Clamp<u16> {
    fn from(_: ()) -> Self {
        Clamp::None
    }
}
impl<T> From<(T, T)> for Clamp<T> {
    fn from(v: (T, T)) -> Self {
        Clamp::MinMax(v.0, v.1)
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margin_sides() {
        let m = Margin(1, 2, 3, 4);

        assert_eq!(m.left(), 1, "Left");
        assert_eq!(m.top(), 2, "Top");
        assert_eq!(m.right(), 3, "Right");
        assert_eq!(m.bottom(), 4, "Bottom");
        assert_eq!(m.hor(), (1, 3), "Horizontal");
        assert_eq!(m.ver(), (2, 4), "Vertical");
    }
    #[test]
    fn into_margin() {
        let all = Margin::from(10);
        let axis = Margin::from((10, 20));
        let each = Margin::from((1, 2, 3, 4));
        
        assert_eq!(all, Margin(10, 10, 10, 10), "All sides");

        assert_eq!(axis.hor(), (10, 10), "Horizontal sides");
        assert_eq!(axis.ver(), (20, 20), "Vertical sides");

        assert_eq!(each, Margin(1, 2, 3, 4), "Each side");
        assert_eq!(each.left(), 1, "Left side");
        assert_eq!(each.top(), 2, "Top side");
        assert_eq!(each.right(), 3, "Right side");
        assert_eq!(each.bottom(), 4, "Bottom side");
    }

    #[test]
    fn clamp() {
        let none = Clamp::<u16>::None;
        let min = Clamp::Min(4);
        let max = Clamp::Max(10);
        let min_max = Clamp::MinMax(4, 10);

        // Set
        assert_eq!(none.set_min(2), Clamp::Min(2));
        assert_eq!(none.set_max(4), Clamp::Max(4));

        assert_eq!(min.set_min(2), Clamp::Min(2));
        assert_eq!(min.set_max(6), Clamp::MinMax(4, 6));

        assert_eq!(max.set_min(2), Clamp::MinMax(2, 10));
        assert_eq!(max.set_max(6), Clamp::Max(6));

        assert_eq!(min_max.set_min(2), Clamp::MinMax(2, 10));
        assert_eq!(min_max.set_max(6), Clamp::MinMax(4, 6));

        // Get
        assert_eq!(none.min(), None);
        assert_eq!(none.max(), None);
        assert_eq!(min.min(), Some(4));
        assert_eq!(min.max(), None);
        assert_eq!(max.min(), None);
        assert_eq!(max.max(), Some(10));
        assert_eq!(min_max.min(), Some(4));
        assert_eq!(min_max.max(), Some(10));
    }
}
