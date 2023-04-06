use crate::Pixel;

pub trait Render {
    fn render(&mut self, dt_ms: f64);
    fn pixels(&self) -> &[Pixel];
}
