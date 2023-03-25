use std::collections::HashMap;
use quicksilver::geom::{Vector, Rectangle, Shape};
use quicksilver::graphics::{Image};

use crate::GameObject;
use crate::wall::Wall;
use crate::weapon::Weapon;

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
    weapon: Option<Weapon>,
    sprite: Rectangle,
    direction: Direction,
    weapon_state: Option<WeaponState>,
    speed: f32,
    image: Image,
}

impl Character{
    pub fn new(position: Vector, size: Vector, new_image: Image, weapon_image: Image) -> Character {
        let new_sprite = Rectangle::new(position, size);
        let new_weapon = Weapon::new(
            Rectangle::new(Vector::new(new_sprite.pos.x + new_sprite.size().x, new_sprite.pos.y - 12.0), 
            Vector::new(24.0, 24.0)),
            24.0,
            weapon_image,
        );

        Character {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: Some(new_weapon),
            weapon_state: Some(WeaponState::Default),
            speed: 2.0,
            image: new_image,
        }
    }

    pub fn new_no_weapon(position: Vector, size: Vector, new_image: Image) -> Character {
        let new_sprite = Rectangle::new(position, size);
        
        Character {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: None,
            weapon_state: None,
            speed: 2.0,
            image: new_image,
        }
    }

    pub fn weapon(&self) -> &Weapon {
        &self.weapon.as_ref().expect("No weapon to get")
    }

    pub fn weapon_state(&self) -> WeaponState {
        self.weapon_state.expect("No weapon_state to get")
    }

    pub fn attack(&mut self) {
        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let new_weapon_state = WeaponState::Attack;
        self.weapon_state = Some(new_weapon_state);
        let new_weapon_position = self.recalculate_weapon_position(self.direction, new_weapon_state);
        self.weapon.as_mut().expect("no weapon to attack with").set_position(new_weapon_position);
    }

    pub fn un_attack(&mut self) {
        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let new_weapon_state = WeaponState::Default;
        self.weapon_state = Some(new_weapon_state);
        let new_weapon_position = self.recalculate_weapon_position(self.direction, new_weapon_state);
        self.weapon.as_mut().expect("no weapon to un_attack with").set_position(new_weapon_position);
        
    }

    pub fn move_up(&mut self, game_map: &Vec<Wall>) {

        self.direction = Direction::Up;
        self.sprite.pos.y -= self.speed;

        for wall in game_map{
            let collision_detected = self.collides_with(wall);
            if collision_detected {
                self.sprite.pos.y += self.speed;
                break
            }
        }

        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let weapon_state = self.weapon_state.expect("This is dumb weapon state should be in the weapon");
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        let weapon = self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist");
        weapon.set_position(new_weapon_position);
    }

    pub fn move_down(&mut self, game_map: &Vec<Wall>) {
        self.sprite.pos.y += self.speed;
        self.direction = Direction::Down;
        
        for wall in game_map{
            let collision_detected = self.collides_with(wall);
            if collision_detected {
                self.sprite.pos.y -= self.speed;
                break
            }
        }

        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let weapon_state = self.weapon_state.expect("This is dumb weapon state should be in the weapon");
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        let weapon = self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist");
        weapon.set_position(new_weapon_position);
    }

    pub fn move_left(&mut self, game_map: &Vec<Wall>) {
        self.sprite.pos.x -= self.speed;
        self.direction = Direction::Left;
        
        for wall in game_map{
            let collision_detected = self.collides_with(wall);
            if collision_detected {
                self.sprite.pos.x += self.speed;
                break
            }
        }

        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let weapon_state = self.weapon_state.expect("This is dumb weapon state should be in the weapon");
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        let weapon = self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist");
        weapon.set_position(new_weapon_position);
    }

    pub fn move_right(&mut self, game_map: &Vec<Wall>) {
        self.sprite.pos.x += self.speed;
        self.direction = Direction::Right;
        
        for wall in game_map{
            let collision_detected = self.collides_with(wall);
            if collision_detected {
                self.sprite.pos.x -= self.speed;
                break
            }
        }

        if self.weapon.is_none() || self.weapon_state.is_none() {
            return;
        }
        let weapon_state = self.weapon_state.expect("This is dumb weapon state should be in the weapon");
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        let weapon = self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist");
        weapon.set_position(new_weapon_position);
    }

    pub fn recalculate_weapon_position(&mut self, direction: Direction, state: WeaponState) -> Vector {
  
        let mut weapon = self.weapon.clone().expect("Can't Calculate the location of a weapon that doesn't exist");
        let weapon_positions = HashMap::from([
            (Direction::Up, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - weapon.size().x / 2.0), 
                    self.sprite.pos.y - weapon.size().y)
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - weapon.size().x / 2.0), 
                    self.sprite.pos.y - weapon.size().y - weapon.range())
                ),
            ])),
            (Direction::Right, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + self.sprite.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - weapon.size().y / 2.0))
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + self.sprite.size().x + weapon.range(), 
                    self.sprite.pos.y + (self.sprite.size.y / 2.0 - weapon.sprite().size.y / 2.0))
                )
            ])),
            (Direction::Down, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - weapon.size().x / 2.0),  
                    self.sprite.pos.y + self.size().y)
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - weapon.size().x / 2.0),  
                    self.sprite.pos.y + self.size().y + weapon.range())
                ),
            ])),
            (Direction::Left, HashMap::from([
                (WeaponState::Default, Vector::new(
                    self.sprite.pos.x - weapon.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - weapon.size().y / 2.0))
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x - weapon.size().x - weapon.range(),
                    self.sprite.pos.y + (self.size().y / 2.0 - weapon.size().y / 2.0))
                ),
            ])),
    
    
        ]);

        weapon_positions[&direction][&state]
    }

    pub fn move_towards(&mut self, target_location: Vector, game_map: &Vec<Wall>) {
    
        if target_location.x < self.sprite.pos.x {
            self.move_left(game_map);
        }
        if target_location.x > self.sprite.pos.x {
            self.move_right(game_map);
        }
        if target_location.y < self.sprite.pos.y {
            self.move_up(game_map);
        }
        if target_location.y > self.sprite.pos.y {
            self.move_down(game_map);
        }
        
    }

    pub fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
    }

}

impl GameObject for Character {
    
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

    fn collides_with<T: GameObject>(&self, other_object: &T) -> bool {
        self.sprite.overlaps_rectangle(&other_object.sprite())
    }

}