use quicksilver::graphics::Image;
use quicksilver::geom::Vector;

use crate::wall::Wall;

pub struct GameMap {
    map: Vec<Wall>
}

impl GameMap {

    pub fn new(wall_image: Image) -> GameMap {
        let mut new_map:Vec<Wall> = Vec::new();
        new_map.push(Wall::new(wall_image, Vector::new(450.0, 600.0)));

        GameMap{
            map: new_map
        }
    }

    pub fn map(&self) -> &Vec<Wall> {
        &self.map
    }
}