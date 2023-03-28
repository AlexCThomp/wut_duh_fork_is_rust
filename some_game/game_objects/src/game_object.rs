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
        image: Image, 
        size: Vector, 
        velocity: Vector,
        range: f32, 
        state: WeaponState, 
        collidable: bool
    ) -> GameObject {
        
        let sprite = Rectangle::new(position, size);

        GameObject {
            sprite,
            direction: Direction::Right,
            velocity,
            acceleration: 0.1,
            max_speed: 4.0,
            image: image.clone(),
            images: HashMap::from([
                (Direction::Up, image.clone()),
                (Direction::Left, image.clone()),
                (Direction::Down, image.clone()),
                (Direction::Right, image.clone()),
            ]),
            collidable,
            range,
            state,
        }
    }

    pub fn new_floor(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, Vector::new(0.0, 0.0), 0.0, WeaponState::Default, false)
    }

    pub fn new_wall(position: Vector, new_image: Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, Vector::new(0.0,0.0), 0.0, WeaponState::Default, true)
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

    pub fn shoot(&mut self, bullets: &mut Vec<GameObject>) {
        let bullet_size = Vector::new(12.0,12.0);
        let bullet_position = self.calculate_bullet_position(self.direction, bullet_size);
        let mut bullet_velocity = Vector::new(0.0,0.0);
        if self.direction == Direction::Right {
            bullet_velocity.x = 12.0;
        }
        if self.direction == Direction::Left {
            bullet_velocity.x = -12.0;
        }
        if self.direction == Direction::Up {
            bullet_velocity.y = -12.0;
        }
        if self.direction == Direction::Down {
            bullet_velocity.y = 12.0;
        }
        bullets.push(
            GameObject::new(
                bullet_position,
                self.image.clone(),
                bullet_size,
                bullet_velocity,
                self.range,
                WeaponState::Attack,
                false,
            )
        )
    }

    pub fn calculate_bullet_position(&mut self, direction: Direction, size: Vector) -> Vector {
  
        let weapon_positions = HashMap::from([
            (Direction::Up, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - size.x / 2.0), 
                    self.sprite.pos.y - size.y)
            ),
            (Direction::Right, Vector::new(
                    self.sprite.pos.x + self.size().x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - size.y / 2.0))
            ),
            (Direction::Down, Vector::new(
                    self.sprite.pos.x + (self.size().x / 2.0 - size.x / 2.0),  
                    self.sprite.pos.y + self.size().y)
            ),
            (Direction::Left, Vector::new(
                    self.sprite.pos.x - size.x, 
                    self.sprite.pos.y + (self.size().y / 2.0 - size.y / 2.0))
            ),
        ]);
        weapon_positions[&direction]
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