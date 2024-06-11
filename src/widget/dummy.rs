use super::Draw;

/// Dummy
/// A widget that does nothing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dummy;
impl Draw for Dummy {
    fn draw(self, _: &mut crate::buffer::Buffer, rect: crate::layout::Rect) -> crate::layout::Rect {
        rect
    }
}
