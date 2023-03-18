// calculate the distance between two points on a plane

use quicksilver::geom::Vector;

pub(crate) fn distance(point1: Vector, point2: Vector) -> f32 {
    let distance = 
         (
            (point1.x - point2.x) * (point1.x - point2.x)
            +
            (point1.y - point2.y) * (point1.y - point2.y)
        ).sqrt();
        
    distance
    
}