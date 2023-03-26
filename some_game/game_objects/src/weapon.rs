use quicksilver::{geom::{Vector, Rectangle, Shape}, graphics::{Image}};

use crate::GameObject;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponState {
    Default,
    Attack
}

#[derive(Clone)]
pub struct Weapon{
    range: f32,
    sprite: Rectangle,
    image: Image,
    collidable: bool,
    state: WeaponState,
}

impl Weapon {

    pub fn new(new_sprite: Rectangle, new_range: f32, new_image: Image) -> Weapon {
        
        Weapon {
            sprite: new_sprite,
            range: new_range,
            image: new_image,
            collidable: true,
            state: WeaponState::Default,
        }
    }

    pub fn set_position(&mut self, new_position: Vector) {
        self.sprite.pos = new_position;
    }

    pub fn range(&mut self) -> f32 {
        self.range
    }

    pub fn state(&self) -> WeaponState {
        self.state
    }

    pub fn set_state(&mut self, new_state: WeaponState) {
        self.state = new_state;
    }

}

impl GameObject for Weapon {
    
    fn image(&self) -> &Image {
        &self.image
    }

    fn set_image(&mut self, new_image: Image) {
        self.image = new_image;
    }

    fn sprite(&self) -> Rectangle {
        self.sprite
    }

    fn size(&self) -> Vector {
        self.sprite.size
    }

    fn position(&self) -> Vector {
        self.sprite.pos
    }

    fn is_collidable(&self) -> bool {
        self.collidable
    }

    fn collides_with<T: GameObject>(&self, other_object: &T) -> bool {
        if other_object.is_collidable(){
            return self.sprite.overlaps_rectangle(&other_object.sprite());
        }
        false
    }

}
