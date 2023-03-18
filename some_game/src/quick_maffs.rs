// calculate the distance between two points on a plane

use quicksilver::geom::{Circle, Rectangle, Vector, Shape};

pub(crate) fn distance(point1: Vector, point2: Vector) -> f32 {
    let distance = 
         (
            (point1.x - point2.x) * (point1.x - point2.x)
            +
            (point1.y - point2.y) * (point1.y - point2.y)
        ).sqrt();
        
    distance
    
}

pub(crate) fn collision_rectangle_circle(circle: Circle, rectangle: Rectangle) -> bool {
    let collision: bool = (distance(
            circle.center(), 
            rectangle.pos
        ) <= circle.radius
    )
    ||
    (distance(
            circle.center(), 
            Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y+rectangle.size.y)
        ) <= circle.radius
    )
    ||
    (distance(
            circle.center(), 
            Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y)
        ) <= circle.radius
    )
    ||
    (distance(
        circle.center(), 
        Vector::new(rectangle.pos.x, rectangle.pos.y+rectangle.size.y)
    ) <= circle.radius);

    collision
}