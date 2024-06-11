use crate::{buffer::Buffer, layout::Rect};

pub trait Draw {
    fn draw(self, buf: &mut Buffer, rect: Rect) -> Rect;
}
pub trait RefDraw {
    fn draw(&self, buf: &mut Buffer, rect: Rect) -> Rect;
}
