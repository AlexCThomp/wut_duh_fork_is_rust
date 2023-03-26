use std::collections::HashMap;
use quicksilver::geom::{Vector, Rectangle, Shape};
use quicksilver::graphics::{Image};

use crate::{GameObject, GameObjectType};
use crate::weapon::{Weapon, WeaponState};

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
    speed: f32,
    image: Image,
    collidable: bool,
}

impl Character{
    pub fn new(position: Vector, new_image: Image, weapon_image: Image) -> Character {
        let size = Vector::new(32.0, 32.0);
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
            speed: 2.0,
            image: new_image,
            collidable: true,
        }
    }

    pub fn new_no_weapon(position: Vector, new_image: Image, is_collidable: bool) -> Character {
        let size = Vector::new(32.0, 32.0);
        let new_sprite = Rectangle::new(position, size);
        
        Character {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: None,
            speed: 2.0,
            image: new_image,
            collidable: is_collidable,
        }
    }

    pub fn new_of_type(position: Vector, new_image: Image, weapon_image: Option<Image>, game_obj_type: GameObjectType) -> Character {
        if game_obj_type == GameObjectType::Player {
            return Character::new(position, new_image, weapon_image.expect("need a weapon image"));
        }
        else if game_obj_type == GameObjectType::Floor {
            return Character::new_no_weapon(position, new_image, false);
        }
        else {
            return Character::new_no_weapon(position, new_image, true);
        }
    }

    pub fn weapon(&self) -> &Weapon {
        &self.weapon.as_ref().expect("No weapon to get")
    }

    pub fn weapon_state(&self) -> WeaponState {
        self.weapon.as_ref().expect("no weapon to get state from").state()
    }

    pub fn set_weapon_state(&mut self, new_state: WeaponState) {
        self.weapon.as_mut().expect("no weapon to set state for").set_state(new_state);
    }

    pub fn attack(&mut self) {
        if self.weapon.is_none() {
            return;
        }
        let new_weapon_state = WeaponState::Attack;
        self.set_weapon_state(new_weapon_state);
        let new_weapon_position = self.recalculate_weapon_position(self.direction, new_weapon_state);
        self.weapon.as_mut().expect("no weapon to attack with").set_position(new_weapon_position);
    }

    pub fn un_attack(&mut self) {
        if self.weapon.is_none() {
            return;
        }
        let new_weapon_state = WeaponState::Default;
        self.set_weapon_state(new_weapon_state);
        let new_weapon_position = self.recalculate_weapon_position(self.direction, new_weapon_state);
        self.weapon.as_mut().expect("no weapon to un_attack with").set_position(new_weapon_position);
        
    }

    pub fn move_up(&mut self, game_map: &Vec<Character>) {

        self.direction = Direction::Up;
        self.sprite.pos.y -= self.speed;

        for map_element in game_map{
            let collision_detected = self.collides_with(map_element);
            if collision_detected {
                self.sprite.pos.y += self.speed;
                break
            }
        }

        if self.weapon.is_none() {
            return;
        }
        let weapon_state = self.weapon_state();
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist").set_position(new_weapon_position);
    }

    pub fn move_down(&mut self, game_map: &Vec<Character>) {
        self.sprite.pos.y += self.speed;
        self.direction = Direction::Down;
        
        for map_element in game_map{
            let collision_detected = self.collides_with(map_element);
            if collision_detected {
                self.sprite.pos.y -= self.speed;
                break
            }
        }

        if self.weapon.is_none() {
            return;
        }
        let weapon_state = self.weapon_state();
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist").set_position(new_weapon_position);
    }

    pub fn move_left(&mut self, game_map: &Vec<Character>) {
        self.sprite.pos.x -= self.speed;
        self.direction = Direction::Left;
        
        for character in game_map{
            let collision_detected = self.collides_with(character);
            if collision_detected {
                self.sprite.pos.x += self.speed;
                break
            }
        }

        if self.weapon.is_none() {
            return;
        }
        let weapon_state = self.weapon_state();
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist").set_position(new_weapon_position);
    }

    pub fn move_right(&mut self, game_map: &Vec<Character>) {
        self.sprite.pos.x += self.speed;
        self.direction = Direction::Right;
        
        for map_element in game_map{
            let collision_detected = self.collides_with(map_element);
            if collision_detected {
                self.sprite.pos.x -= self.speed;
                break
            }
        }

        if self.weapon.is_none() {
            return;
        }
        let weapon_state = self.weapon_state();
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        self.weapon.as_mut().expect("Somehow you're trying to move a weapon that doesn't exist").set_position(new_weapon_position);
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

    pub fn move_towards(&mut self, target_location: Vector, game_map: &Vec<Character>) {
    
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