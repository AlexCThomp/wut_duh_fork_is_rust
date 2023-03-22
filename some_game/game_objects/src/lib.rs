use quicksilver::{graphics::Color, geom::{Rectangle, Vector}};

pub mod character;
pub mod weapon;

pub trait GameObject {
    fn color(&self) -> Color;
    fn set_color(&mut self, new_color: Color);
    fn sprite(&self) -> Rectangle;
    fn size(&self) -> Vector;
    fn position(&self) -> Vector;
    fn collides_with<T: GameObject>(&self, _:&T) -> bool;
}