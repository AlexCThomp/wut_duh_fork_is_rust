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
    sprite: Rectangle,
    direction: Direction,
    velocity: Vector,
    acceleration: f32,
    max_speed: f32,
    images: HashMap<Direction, Image>,
    image: Image,
    collidable: bool,
    state: WeaponState,
    range: f32,
}

impl GameObject{

    pub fn new(
        position: Vector, 
        new_image: Image, 
        size: Vector, 
        new_range: f32, 
        new_weapon_state: WeaponState, 
        is_collidable: bool
    ) -> GameObject {
        
        let new_sprite = Rectangle::new(position, size);

        GameObject {
            sprite: new_sprite,
            direction: Direction::Right,
            velocity: Vector::new(0.0,0.0),
            acceleration: 0.1,
            max_speed: 4.0,
            image: new_image.clone(),
            images: HashMap::from([
                (Direction::Up, new_image.clone()),
                (Direction::Left, new_image.clone()),
                (Direction::Down, new_image.clone()),
                (Direction::Right, new_image.clone()),
            ]),
            collidable: is_collidable,
            range: new_range,
            state: new_weapon_state,
        }
    }

    pub fn new_floor(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, 0.0, WeaponState::Default, false)
    }

    pub fn new_wall(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, 0.0, WeaponState::Default, true)
    }

    pub fn new_with_direction(
        position: Vector, 
        up_image: Image,
        left_image: Image,
        down_image: Image,
        right_image: Image, 
        ) -> GameObject {

        let size = Vector::new(32.0, 32.0);
        let new_sprite = Rectangle::new(position, size);

        GameObject {
            sprite: new_sprite,
            direction: Direction::Right,
            velocity: Vector::new(0.0,0.0),
            acceleration: 0.1,
            max_speed: 4.0,
            images: HashMap::from([
                (Direction::Up, up_image.clone()),
                (Direction::Left, left_image.clone()),
                (Direction::Down, down_image.clone()),
                (Direction::Right, right_image.clone()),
            ]),
            image: right_image.clone(),
            collidable: true,
            state: WeaponState::Default,
            range: 0.0,
        }

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

    pub fn set_direction(&mut self, new_direction: Direction) {

        self.direction = new_direction;
        self.image = self.images[&new_direction].clone();
        
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