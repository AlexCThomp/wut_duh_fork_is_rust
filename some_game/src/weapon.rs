use quicksilver::geom::{Vector, Rectangle, Shape};

#[derive(Copy, Clone)]
pub struct Weapon{
    range: f32,
    sprite: Rectangle,
}



impl Weapon {

    pub fn new(new_sprite: Rectangle, new_range: f32) -> Weapon {
        
        Weapon {
            sprite: new_sprite,
            range: new_range,
        }
    }

    // pub fn set_direction(&mut self, new_direction: Direction) {
    //     self.direction = new_direction;
    //     self.position = self.directions[&self.direction][&WeaponState::Default];
    // }
    
    // pub fn attack(&mut self) {
    //     self.position = self.directions[&self.direction][&WeaponState::Attack];
    // }

    // pub fn reset(&mut self) {
    //     self.position = self.directions[&self.direction][&WeaponState::Default];
    // }

    pub fn set_position(&mut self, new_position: Vector) {
        self.sprite.pos = new_position;
    }

    pub fn sprite(&mut self) -> Rectangle {
        self.sprite
    }

    pub fn size(&mut self) -> Vector {
        self.sprite.size
    }

    pub fn range(&mut self) -> f32 {
        self.range
    }
    
    pub fn collides_with(&mut self, other_sprite: Rectangle) -> bool {
        self.sprite.overlaps_rectangle(&other_sprite)
    }



}
