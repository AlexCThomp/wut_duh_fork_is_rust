use std::collections::HashMap;
use std::num::NonZeroUsize;
use quicksilver::Timer;
use quicksilver::geom::{Vector, Rectangle, Shape, Circle};
use quicksilver::graphics::{Image};
use rand::Rng;

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

#[derive(Clone)]
pub struct GameObject {
    weapon: Option<Box<GameObject>>,
    start_position: Vector,
    sprite: Rectangle,
    direction: Vector,
    velocity: Vector,
    acceleration: Vector,
    max_speed: f32,
    images: HashMap<Direction, Image>,
    image: Image,
    collidable: bool,
    state: WeaponState,
    range: f32,
    shoot_rate: Timer,
}

impl GameObject{

    pub fn new(
        position: Vector,
        image: &Image, 
        size: Vector, 
        velocity: Vector,
        range: f32, 
        state: WeaponState, 
        collidable: bool
    ) -> GameObject {
        
        let sprite = Rectangle::new(position, size);

        GameObject {
            weapon: None,
            start_position: position,
            sprite,
            direction: Vector::new(1.0, 0.0),
            velocity,
            acceleration: Vector::new(0.0, 0.0),
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
            shoot_rate: Timer::time_per_second(4.0),
        }
    }

    pub fn new_with_weapon(
        position: Vector, 
        up_image: &Image,
        left_image: &Image,
        down_image: &Image,
        right_image: &Image,
        weapon_image: &Image
    ) -> GameObject {
        let mut new_object = GameObject::new_with_direction(
            position, up_image, left_image, down_image, right_image
        );

        let weapon_size = Vector::new(12.0, 12.0);
        let new_weapon = GameObject::new_weapon(new_object.calculate_weapon_position(weapon_size), weapon_image);
        new_object.set_weapon(new_weapon);

        new_object
    }

    pub fn new_random_enemy(image: &Image) -> GameObject {

        let x_coord = rand::thread_rng().gen_range(200..800) as f32;
        let y_coord = rand::thread_rng().gen_range(100..700) as f32;

        GameObject::new(
            Vector::new(x_coord, y_coord), 
            image,
            Vector::new(12.0, 12.0),
            Vector::new(0.0,0.0),
            0.0,
            WeaponState::Attack,
            true,
        )
    }

    pub fn new_weapon(position: Vector, new_image: &Image) -> GameObject {
        let size = Vector::new(12.0, 12.0);
        GameObject::new(position, new_image, size, Vector::new(0.0, 0.0), 0.0, WeaponState::Default, false)
    }

    pub fn new_floor(position: Vector, new_image: &Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, Vector::new(0.0, 0.0), 0.0, WeaponState::Default, false)
    }

    pub fn new_wall(position: Vector, new_image: &Image) -> GameObject {
        let size = Vector::new(32.0, 32.0);
        GameObject::new(position, new_image, size, Vector::new(0.0,0.0), 0.0, WeaponState::Default, true)
    }

    pub fn new_with_direction(
        position: Vector, 
        up_image: &Image,
        left_image: &Image,
        down_image: &Image,
        right_image: &Image, 
        ) -> GameObject {

        let size = Vector::new(32.0, 32.0);
        let new_sprite = Rectangle::new(position, size);

        GameObject {
            weapon: None,
            start_position: position,
            sprite: new_sprite,
            direction: Vector::new(1.0, 0.0),
            velocity: Vector::new(0.0,0.0),
            acceleration: Vector::new(0.0, 0.0),
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
            range: 300.0,
            shoot_rate: Timer::time_per_second(2.0),
        }

    }

    pub fn move_up(&mut self) {

        self.image = self.images[&Direction::Up].clone();
        let new_velocity = self.velocity.y - 0.1;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.y = new_velocity;
        }
    }

    pub fn move_down(&mut self) {

        self.image = self.images[&Direction::Down].clone();
        let new_velocity = self.velocity.y + 0.1;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.y = new_velocity;
        }
    }

    pub fn move_left(&mut self) {

        self.image = self.images[&Direction::Left].clone();
        let new_velocity = self.velocity.x - 0.1;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.x = new_velocity;
        }
    }

    pub fn move_right(&mut self) {

        self.image = self.images[&Direction::Right].clone();
        let new_velocity = self.velocity.x + 0.1;
        if new_velocity.abs() <= self.max_speed {
            self.velocity.x = new_velocity;
        }
        
    }

    pub fn shoot(&mut self, bullets: &mut Vec<GameObject>) {
        
        if self.shoot_rate.exhaust() >= NonZeroUsize::new(1) {
            bullets.push(
                GameObject::new(
                    self.weapon().position(),
                    self.weapon().image(),
                    self.weapon().size(),
                    self.direction*8.0,
                    self.range,
                    WeaponState::Attack,
                    false,
                )
            );
            self.shoot_rate.reset();
        }
    }

    pub fn calculate_weapon_position(&self, weapon_size: Vector) -> Vector {
        
        // compute minimum radius that subsumes this object
        let object_radius = ((self.size().x/2.0).powf(2.0) + (self.size().x/2.0).powf(2.0)).sqrt() + weapon_size.x;
        
        //compute direction magnitude
        let direction_magnitude = (self.direction.x.powf(2.0) + self.direction.y.powf(2.0)).sqrt();

        //unit direction mag and object rad ratio
        // this divides by zero and then... nothing happens?
        // the weapon disappears and can't fire bullets (fine with me... but why?)
        let direction_mult = object_radius/direction_magnitude;
    
        
        // place weapon at center of this object
        let mut weapon_position = self.center();
        
        // displace in this objects direction by the radius
        let displace_vector = self.direction * direction_mult;
        weapon_position += displace_vector;

        // center the weapon
        weapon_position.x -= weapon_size.x/2.0;
        weapon_position.y -= weapon_size.y/2.0;

        return weapon_position;

    }


    pub fn move_towards(&mut self, target_location: Vector) {
    
        if target_location.x < self.position().x {
            self.move_left();
        }
        if target_location.x > self.position().x {
            self.move_right();
        }
        if target_location.y < self.position().y {
            self.move_up();
        }
        if target_location.y > self.position().y {
            self.move_down();
        }
        
    }

    pub fn carry_momentum(&mut self, game_map: &Vec<GameObject>) {

        self.sprite.pos.x += self.velocity.x;
        if self.check_collisions(game_map){
            self.sprite.pos.x -= self.velocity.x;
            self.velocity.x = 0.0;
        }

        self.sprite.pos.y += self.velocity.y;
        if self.check_collisions(game_map){
            self.sprite.pos.y -= self.velocity.y;
            self.velocity.y = 0.0;
        }
        if self.weapon.is_some() {
            let new_weapon_position = self.calculate_weapon_position(self.weapon().size());
            self.weapon.as_mut().expect("no weapon carry momentum").set_position(new_weapon_position);
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

    pub fn update_direction(&mut self, new_direction: Direction) {

        // formula: direction.x^2 + direction.y^2 = 1

        if new_direction == Direction::Left && self.direction.x > -1.0{
            self.direction.x -= 0.05;
            let y_mag = 1.0 - (self.direction.x * self.direction.x);
            if self.direction.y >= 0.0 { self.direction.y = y_mag }
            else { self.direction.y = -y_mag }
        }
        if new_direction == Direction::Right && self.direction.x < 1.0{
            self.direction.x += 0.05;
            let y_mag = 1.0 - (self.direction.x * self.direction.x);
            if self.direction.y >= 0.0 { self.direction.y = y_mag }
            else { self.direction.y = -y_mag }
        }
        if new_direction == Direction::Up && self.direction.y > -1.0{
            self.direction.y -= 0.05;
            let x_mag = 1.0 - (self.direction.y * self.direction.y);
            if self.direction.x >= 0.0 { self.direction.x = x_mag }
            else { self.direction.x = -x_mag }
        }
        if new_direction == Direction::Down && self.direction.y < 1.0{
            self.direction.y += 0.05;
            let x_mag = 1.0 - (self.direction.y * self.direction.y);
            if self.direction.x >= 0.0 { self.direction.x = x_mag }
            else { self.direction.x = -x_mag }
        }
        
    }

    pub fn set_direction(&mut self, tilt: Vector) {
        
        self.direction = tilt;

    }

    pub fn got_shot(&self, bullets: &Vec<GameObject>) -> bool {

        for bullet in bullets{
            if self.sprite.overlaps_rectangle(&bullet.sprite()){
                return true;
            }
        }
        return false;

    }

    pub fn out_of_range(&self) -> bool {
        let range_radius = Circle::new(self.start_position, self.range);
        !self.sprite().overlaps_circle(&range_radius)
    }

    pub fn set_weapon(&mut self, new_weapon: GameObject) {
        self.weapon = Some(Box::new(new_weapon));
    }

    pub fn weapon(&self) -> &GameObject {
        &self.weapon.as_ref().expect("No weapon to get")
    }

    pub fn accelerate(&mut self) {

        let new_velocity_x = self.velocity.x + self.acceleration.x;
        if new_velocity_x.abs() <= self.max_speed {
            self.velocity.x = new_velocity_x;
        }

        let new_velocity_y = self.velocity.y + self.acceleration.y;
        if new_velocity_y.abs() <= self.max_speed {
            self.velocity.y = new_velocity_y;
        }
        
    }

    pub fn set_acceleration(&mut self, tilt: Vector) {
        
        self.acceleration = tilt*0.1;
        
    }

    pub fn acceleration(&self) -> Vector {
        return self.acceleration;
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

    pub fn center(&self) -> Vector {

        let center = Vector::new(
            self.position().x + (self.size().x/2.0), 
            self.position().y + (self.size().y/2.0));

        return center;

    }

    pub fn is_collidable(&self) -> bool {
        self.collidable
    }

    pub fn collides_with(&self, other_object: &GameObject) -> bool {
        if other_object.is_collidable() && self.is_collidable() {
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

    pub fn set_range(&mut self, new_range: f32) {
        self.range = new_range;
    }

    pub fn state(&self) -> WeaponState {
        self.state
    }

    pub fn set_state(&mut self, new_state: WeaponState) {
        self.state = new_state;
    }

}