use std::collections::HashMap;
use quicksilver::geom::{Vector, Rectangle, Shape};
use quicksilver::graphics::{Image};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponState {
    Default,
    Attack
}

pub struct GameObject {
    weapon: Option<Box<GameObject>>,
    sprite: Rectangle,
    direction: Direction,
    weight: f32,
    velocity: Vector,
    acceleration: f32,
    max_speed: f32,
    image: Image,
    collidable: bool,
    state: WeaponState,
    range: f32,
}

impl GameObject{

    pub fn new(position: Vector, new_image: Image, size: Vector, new_range: f32, new_weapon_state: WeaponState, is_collidable: bool) -> GameObject {
        let new_sprite = Rectangle::new(position, size);
        
        GameObject {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: None,
            weight: 4.0,
            velocity: Vector::new(0.0,0.0),
            acceleration: 0.1,
            max_speed: 4.0,
            image: new_image,
            collidable: is_collidable,
            range: new_range,
            state: new_weapon_state,
        }
    }

    pub fn new_weapon(position: Vector, new_image: Image, range: f32) -> GameObject {
        let size = Vector::new(24.0, 24.0);
        GameObject::new(position, new_image, size, range, WeaponState::Default, false)
    }

    pub fn new_floor(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, 0.0, WeaponState::Default, false)
    }

    pub fn new_wall(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, 0.0, WeaponState::Default, true)
    }

    pub fn new_with_weapon(position: Vector, new_image: Image, weapon_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        let new_sprite = Rectangle::new(position, size);
        let new_weapon = GameObject::new_weapon(
            Vector::new(new_sprite.pos.x + new_sprite.size().x, new_sprite.pos.y - 12.0), 
            weapon_image,
            24.0
        );

        GameObject {
            sprite: new_sprite,
            direction: Direction::Right,
            weapon: Some(Box::new(new_weapon)),
            weight: 4.0,
            velocity: Vector::new(0.0,0.0),
            acceleration: 0.1,
            max_speed: 4.0,
            image: new_image,
            collidable: true,
            state: WeaponState::Default,
            range: 0.0,
        }
    }

    pub fn weapon(&self) -> &GameObject {
        &self.weapon.as_ref().expect("No weapon to get")
    }

    pub fn set_weapon(&mut self, new_weapon: GameObject) {
        self.weapon = Some(Box::new(new_weapon));
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

    pub fn move_up(&mut self) {

        let new_velocity = self.velocity.y - self.acceleration;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.y = new_velocity;
        }
    }

    pub fn move_down(&mut self) {

        let new_velocity = self.velocity.y + self.acceleration;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.y = new_velocity;
        }
    }

    pub fn move_left(&mut self) {

        let new_velocity = self.velocity.x - self.acceleration;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.x = new_velocity;
        }
    }

    pub fn move_right(&mut self) {

        let new_velocity = self.velocity.x + self.acceleration;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.x = new_velocity;
        }
        
    }

    pub fn recalculate_weapon_position(&mut self, direction: Direction, state: WeaponState) -> Vector {
  
        let weapon = self.weapon.as_ref().expect("Can't Calculate the location of a weapon that doesn't exist");
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
                    self.sprite.pos.x + self.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - weapon.size().y / 2.0))
                ),
                (WeaponState::Attack, Vector::new(
                    self.sprite.pos.x + self.size().x + weapon.range(), 
                    self.sprite.pos.y + (self.size().y / 2.0 - weapon.sprite().size.y / 2.0))
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

    pub fn move_towards(&mut self, target_location: Vector) {
    
        if target_location.x < self.sprite.pos.x {
            self.move_left();
        }
        if target_location.x > self.sprite.pos.x {
            self.move_right();
        }
        if target_location.y < self.sprite.pos.y {
            self.move_up();
        }
        if target_location.y > self.sprite.pos.y {
            self.move_down();
        }
        
    }

    pub fn carry_momentum(&mut self, game_map: &Vec<GameObject>) {

        self.sprite.pos.x += self.velocity.x;
        if self.check_collisions(game_map){
            self.sprite.pos.x -= self.velocity.x;
        }

        self.sprite.pos.y += self.velocity.y;
        if self.check_collisions(game_map){
            self.sprite.pos.y -= self.velocity.y;
        }
    }

    fn check_collisions(&self, game_map: &Vec<GameObject>) -> bool{

        let mut collision_detected: bool = false;
        for map_element in game_map{
            collision_detected = self.collides_with(map_element);
            if collision_detected {
                break
            }
        }
        collision_detected
    }

    pub fn fall(&mut self, game_map: &Vec<GameObject>) {
        self.sprite.pos.y += self.weight;

        for map_element in game_map{
            let collision_detected = self.collides_with(map_element);
            if collision_detected {
                self.sprite.pos.y -= self.weight;
                break
            }
        }
    }

    pub fn set_direction(&mut self, new_direction: Direction) {

        self.direction = new_direction;

        if self.weapon.is_none() {
            return;
        }
        let weapon_state = self.weapon_state();
        let new_weapon_position = self.recalculate_weapon_position(self.direction, weapon_state);
        self.weapon.as_mut().expect("set_direction() is trying to move a weapon that doesn't exist").set_position(new_weapon_position);
    }

    pub fn set_speed(&mut self, new_speed: f32) {
        self.acceleration = new_speed;
    }

    pub fn image(&self) -> &Image {
        &self.image
    }

    pub fn set_image(&mut self, new_image: Image) {
        self.image = new_image;
    }

    pub fn sprite(&self) -> Rectangle {
        self.sprite
    }

    pub fn size(&self) -> Vector {
        self.sprite.size
    }

    pub fn position(&self) -> Vector {
        self.sprite.pos
    }

    pub fn is_collidable(&self) -> bool {
        self.collidable
    }

    pub fn collides_with(&self, other_object: &GameObject) -> bool {
        if other_object.is_collidable(){
            return self.sprite.overlaps_rectangle(&other_object.sprite());
        }
        false
    }

    pub fn set_position(&mut self, new_position: Vector) {
        self.sprite.pos = new_position;
    }

    pub fn range(&self) -> f32 {
        self.range
    }

    pub fn state(&self) -> WeaponState {
        self.state
    }

    pub fn set_state(&mut self, new_state: WeaponState) {
        self.state = new_state;
    }

}