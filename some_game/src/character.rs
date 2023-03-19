use std::collections::HashMap;
use quicksilver::geom::{Vector, Rectangle, Shape};

use crate::weapon::Weapon;
// use crate::weapon::Weapon;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponState {
    Default,
    Attack
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

pub struct Character {
    weapon: Weapon,
    sprite: Rectangle,
    direction: Direction,
    weapon_state: WeaponState,
    speed: f32,
}

impl Character{
    pub fn new() -> Character{
        let new_sprite = Rectangle::new(Vector::new(300.0, 300.0), Vector::new(32.0, 32.0));
        let new_weapon = Weapon::new(
            Rectangle::new(Vector::new(new_sprite.pos.x + new_sprite.size().x, new_sprite.pos.y - 12.0), 
            Vector::new(24.0, 24.0)),
            24.0);

        Character {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: new_weapon,
            weapon_state: WeaponState::Default,
            speed: 2.0,
        }
    }

    pub fn sprite(&mut self) -> Rectangle {
        self.sprite
    }

    pub fn size(&mut self) -> Vector {
        self.sprite.size
    }

    pub fn weapon(&mut self) -> Weapon {
        self.weapon
    }

    pub fn weapon_state(&mut self) -> WeaponState {
        self.weapon_state
    }

    pub fn attack(&mut self) {
        self.weapon_state = WeaponState::Attack;
        let new_weapon_position = self.recalculate_weapon_position(self.direction, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn un_attack(&mut self) {
        self.weapon_state = WeaponState::Default;
        let new_weapon_position = self.recalculate_weapon_position(self.direction, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn move_up(&mut self) {
        self.sprite.pos.y -= self.speed;
        self.direction = Direction::Up;
        let new_weapon_position = self.recalculate_weapon_position(self.direction, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn move_down(&mut self) {
        self.sprite.pos.y += self.speed;
        self.direction = Direction::Down;
        let new_weapon_position = self.recalculate_weapon_position(Direction::Down, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn move_left(&mut self) {
        self.sprite.pos.x -= self.speed;
        self.direction = Direction::Left;
        let new_weapon_position = self.recalculate_weapon_position(self.direction, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn move_right(&mut self) {
        self.sprite.pos.x += self.speed;
        self.direction = Direction::Right;
        let new_weapon_position = self.recalculate_weapon_position(self.direction, self.weapon_state);
        self.weapon.set_position(new_weapon_position);
    }

    pub fn recalculate_weapon_position(&mut self, direction: Direction, state: WeaponState) -> Vector {
        let weapon_positions = HashMap::from([
            (Direction::Up, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - self.weapon.size().x / 2.0), 
                    self.sprite.pos.y - self.weapon.size().y)
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - self.weapon.size().x / 2.0), 
                    self.sprite.pos.y - self.weapon.size().y - self.weapon.range())
                ),
            ])),
            (Direction::Right, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + self.sprite.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - self.weapon.size().y / 2.0))
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + self.sprite.size().x + self.weapon.range(), 
                    self.sprite.pos.y + (self.sprite.size.y / 2.0 - self.weapon.sprite().size.y / 2.0))
                )
            ])),
            (Direction::Down, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - self.weapon.size().x / 2.0),  
                    self.sprite.pos.y + self.size().y)
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - self.weapon.size().x / 2.0),  
                    self.sprite.pos.y + self.size().y + self.weapon.range())
                ),
            ])),
            (Direction::Left, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x - self.weapon.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - self.weapon.size().y / 2.0))
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x - self.weapon.size().x - self.weapon.range(),
                    self.sprite.pos.y + (self.size().y / 2.0 - self.weapon.size().y / 2.0))
                ),
            ])),
    
    
        ]);

        weapon_positions[&direction][&state]
    }

    pub fn collides_with(&mut self, other_sprite: Rectangle) -> bool {
        self.sprite.overlaps_rectangle(&other_sprite)
    }

}
