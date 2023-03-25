use quicksilver::{graphics::{Image}, geom::{Rectangle, Vector}};

pub mod character;
pub mod weapon;
pub mod game_map;


#[derive(PartialEq, Copy, Clone)]
pub enum GameObjectType {
    Enemy,
    Player,
    Weapon,
    Wall,
    Floor,
}


pub trait GameObject {
    fn image(&self) -> &Image;
    fn set_image(&mut self, new_image: Image);
    fn sprite(&self) -> Rectangle;
    fn size(&self) -> Vector;
    fn position(&self) -> Vector;
    fn is_collidable(&self) -> bool;
    fn collides_with<T: GameObject>(&self, other_object: &T) -> bool;
}