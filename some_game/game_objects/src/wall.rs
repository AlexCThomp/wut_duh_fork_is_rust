use quicksilver::geom::{Vector, Rectangle, Shape};
use quicksilver::graphics::{Image};

use crate::GameObject;

pub struct Wall {
    hit_box: Rectangle,
    image: Image,
}

impl Wall{
    
    pub fn new(new_image: Image, position: Vector) -> Wall {
        
        let new_hit_box = Rectangle::new(position, Vector { x: 32.0, y: 32.0 });

        Wall {
            hit_box: new_hit_box,
            image: new_image,
        }

    }
}

impl GameObject for Wall {
    
    fn image(&self) -> &Image {
        &self.image
    }

    fn set_image(&mut self, new_image: Image) {
        self.image = new_image;
    }

    fn sprite(&self) -> Rectangle {
        self.hit_box
    }

    fn size(&self) -> Vector {
        self.hit_box.size
    }

    fn position(&self) -> Vector {
        self.hit_box.pos
    }

    fn collides_with<T: GameObject>(&self, other_object: &T) -> bool {
        self.hit_box.overlaps_rectangle(&other_object.sprite())
    }

}