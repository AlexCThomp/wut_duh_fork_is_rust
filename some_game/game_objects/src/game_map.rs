use quicksilver::graphics::Image;
use quicksilver::geom::Vector;

use crate::GameObjectType;
use crate::game_object::GameObject;

const MAP_1: [[GameObjectType;32];23] = [
    [GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Floor, GameObjectType::Wall],
    [GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall, GameObjectType::Wall],
    
];

pub struct GameMap {
    map: Vec<GameObject>
}

impl GameMap {

    pub fn new(wall_image: Image, floor_image: Image) -> GameMap {
        let mut new_map:Vec<GameObject> = Vec::new();
        for (y, el) in MAP_1.iter().enumerate() {
            for (x, obj_type) in el.iter().enumerate(){
                let mut x_coord = x as f32;
                let mut y_coord = y as f32;
                x_coord *= 32.0;
                y_coord *= 32.0;
                let position = Vector::new(x_coord, y_coord);

                if *obj_type == GameObjectType::Floor {

                    new_map.push(GameObject::new_of_type(
                        position,
                        floor_image.clone(),
                        None,
                        *obj_type
                    ));

                }
                else if *obj_type == GameObjectType::Wall {

                    new_map.push(GameObject::new_of_type(
                        position,
                        wall_image.clone(),
                        None,
                        *obj_type
                    ));
                    
                }
            }
        }
        

        GameMap{
            map: new_map
        }
    }

    pub fn map(&self) -> &Vec<GameObject> {
        &self.map
    }
}