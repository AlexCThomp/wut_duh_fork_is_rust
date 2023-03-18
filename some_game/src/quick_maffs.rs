// calculate the distance between two points on a plane

use std::mem::discriminant;

use quicksilver::geom::{Circle, Rectangle, Vector, Shape, Line};

pub(crate) fn distance_point_to_point(point1: Vector, point2: Vector) -> f32 {
    let distance = 
         (
            (point1.x - point2.x) * (point1.x - point2.x)
            +
            (point1.y - point2.y) * (point1.y - point2.y)
        ).sqrt();
        
    distance
    
}


pub(crate) fn collision_line_circle(circle: Circle, line: Line) -> bool {

    let ax: f32 = line.a.x - circle.center().x;
    let ay: f32 = line.a.y - circle.center().y;
    let bx: f32 = line.b.x - circle.center().x;
    let by: f32 = line.b.y - circle.center().y;
    let r: f32 = circle.radius;

    let a: f32 = ((bx - ax) * (bx - ax)) + ((by - ay) * (by - ay));
    let b: f32 = 2.0 * (ax * (bx - ax) + ay * (by-ay));
    let c: f32 = ax * ax + ay * ay - r * r;
    let disc: f32 = b * b - 4.0 * a * c;
    if disc <= 0.0 {
        return false;
    }
    let sqrtdisc: f32 = disc.sqrt();
    let t1: f32 = (-b + sqrtdisc)/(2.0 * a);
    let t2: f32 = (-b - sqrtdisc)/(2.0 * a);

    (0.0 < t1 && t1 < 1.0) || (0.0 < t2 && t2 < 1.0)
}


pub(crate) fn collision_rectangle_circle(circle: Circle, rectangle: Rectangle) -> bool {
    
    let rectangle_left_line: Line = Line { 
        a: Vector::new(rectangle.pos.x, rectangle.pos.y), 
        b: Vector::new(rectangle.pos.x, rectangle.pos.y+rectangle.size.y), 
        t: (0.0) 
    };

    let rectangle_top_line: Line = Line { 
        a: Vector::new(rectangle.pos.x, rectangle.pos.y), 
        b: Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y), 
        t: (0.0) 
    };

    let rectangle_bottom_line: Line = Line { 
        a: Vector::new(rectangle.pos.x, rectangle.pos.y+rectangle.size.y), 
        b: Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y+rectangle.size.y), 
        t: (0.0) 
    };

    let rectangle_right_line: Line = Line { 
        a: Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y), 
        b: Vector::new(rectangle.pos.x+rectangle.size.x, rectangle.pos.y+rectangle.size.y), 
        t: (0.0) 
    };

    let collides: bool = 
        collision_line_circle(circle, rectangle_top_line)
        ||
        collision_line_circle(circle, rectangle_bottom_line)
        ||
        collision_line_circle(circle, rectangle_left_line)
        ||
        collision_line_circle(circle, rectangle_right_line);
    
    collides
    
}