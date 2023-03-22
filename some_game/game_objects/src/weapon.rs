use quicksilver::{geom::{Vector, Rectangle, Shape}, graphics::Color};

use crate::GameObject;

#[derive(Copy, Clone)]
pub struct Weapon{
    range: f32,
    sprite: Rectangle,
    color: Color,
}



impl Weapon {

    pub fn new(new_sprite: Rectangle, new_range: f32, new_color: Color) -> Weapon {
        
        Weapon {
            sprite: new_sprite,
            range: new_range,
            color: new_color,
        }
    }

    pub fn set_position(&mut self, new_position: Vector) {
        self.sprite.pos = new_position;
    }

    pub fn range(&mut self) -> f32 {
        self.range
    }
}

impl GameObject for Weapon {
    
    fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
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

    fn collides_with<T: GameObject>(&self, other_object: &T) -> bool {
        self.sprite.overlaps_rectangle(&other_object.sprite())
    }

}
