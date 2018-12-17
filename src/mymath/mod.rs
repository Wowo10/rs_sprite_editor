use sdl2::rect::Point;

fn vector_from_to(from: Point, to: Point) -> Point {
    Point::new(to.x - from.x, to.y - from.y)
}

fn dot_product(first: Point, second: Point) -> i32 {
    first.x * second.x + first.y * second.y
}

fn sign(integer: i32) -> i32 {
    if integer > 0 {
        1
    } else if integer < 0 {
        -1
    } else {
        0
    }
}

fn different_signs(dot_first: i32, dot_second: i32) -> bool {
    sign(dot_first) != sign(dot_second)
}

fn check_wall(wall_first: Point, wall_second: Point, point: Point) -> bool {
    let origin = Point::new(0, 0);

    let ab = vector_from_to(wall_first, wall_second);
    let bc = vector_from_to(wall_first, point);
    let bd = vector_from_to(wall_first, origin);

    let cd = vector_from_to(point, origin);
    let da = vector_from_to(origin, wall_first);
    let db = vector_from_to(origin, wall_second);

    different_signs(dot_product(ab, bc), dot_product(ab, bd))
        && different_signs(dot_product(cd, da), dot_product(cd, db))
}

pub fn check_rect(rect: &sdl2::rect::Rect, point: Point) -> bool {
    let mut counter: i8 = check_wall(rect.top_left(), rect.top_right(), point) as i8;
    counter += check_wall(rect.top_right(), rect.bottom_right(), point) as i8;
    counter += check_wall(rect.bottom_right(), rect.bottom_left(), point) as i8;
    counter += check_wall(rect.bottom_left(), rect.top_left(), point) as i8;

    counter % 2 == 1
}

fn PointInTriangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    // Compute vectors
    let v0 = vector_from_to(a, c);
    let v1 = vector_from_to(b, a);
    let v2 = vector_from_to(p, a);

    // Compute dot products
    let dot00 = dot_product(v0, v0);
    let dot01 = dot_product(v0, v1);
    let dot02 = dot_product(v0, v2);
    let dot11 = dot_product(v1, v1);
    let dot12 = dot_product(v1, v2);

    // Compute barycentric coordinates
    let invDenom = 1 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * invDenom;
    let v = (dot00 * dot12 - dot01 * dot02) * invDenom;

    // Check if point is in triangle
    if u >= 0 && v >= 0 && (u + v) < 1 {
        true
    } else {
        false
    }
}

pub fn PointInRectangle(x: Point, y: Point, z: Point, w: Point, p: Point) -> bool {
    if (PointInTriangle(x, y, z, p)) {
        return true;
    };
    if (PointInTriangle(x, z, w, p)) {
        return true;
    };
    false
}
