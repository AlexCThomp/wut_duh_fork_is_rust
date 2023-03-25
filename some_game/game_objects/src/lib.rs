use quicksilver::{graphics::{Image}, geom::{Rectangle, Vector}};

pub mod character;
pub mod weapon;

pub trait GameObject {
    fn image(&self) -> &Image;
    fn set_image(&mut self, new_image: Image);
    fn sprite(&self) -> Rectangle;
    fn size(&self) -> Vector;
    fn position(&self) -> Vector;
    fn collides_with<T: GameObject>(&self, other_object:&T) -> bool;
}