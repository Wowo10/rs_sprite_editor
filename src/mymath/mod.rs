use sdl2::rect::Point;

// fn vector_from_to(from: Point, to: Point) -> Point {
//     Point::new(to.x - from.x, to.y - from.y)
// }

// fn dot_product(first: Point, second: Point) -> i32 {
//     first.x * second.x + first.y * second.y
// }

// fn sign(integer: i32) -> i32 {
//     if integer > 0 {
//         1
//     } else if integer < 0 {
//         -1
//     } else {
//         0
//     }
// }

// fn different_signs(dot_first: i32, dot_second: i32) -> bool {
//     sign(dot_first) != sign(dot_second)
// }

fn on_segment(p: Point, q: Point, r: Point) -> bool {
    (q.x <= std::cmp::max(p.x, r.x)
        && q.x >= std::cmp::min(p.x, r.x)
        && q.y <= std::cmp::max(p.y, r.y)
        && q.y >= std::cmp::min(p.y, r.y))
}

fn orientation(p: Point, q: Point, r: Point) -> i8 {
    let dot: i32 = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if dot == 0 {
        0
    } else {
        if dot > 0 {
            1
        } else {
            2
        }
    }
}

fn do_intersect(p1: Point, q1: Point, p2: Point) -> bool {
    let q2 = Point::new(0, 0);

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    return false; // Doesn't fall in any of the above cases
}

pub fn check_rect(rect: &sdl2::rect::Rect, point: Point) -> bool {
    let counter: i8 = do_intersect(rect.top_left(), rect.top_right(), point) as i8
        + do_intersect(rect.top_right(), rect.bottom_right(), point) as i8
        + do_intersect(rect.bottom_right(), rect.bottom_left(), point) as i8
        + do_intersect(rect.bottom_left(), rect.top_left(), point) as i8;

    counter % 2 == 1
}

pub fn check_rect2(rect: [Point; 4], point: Point) -> bool {
    let counter: i8 = do_intersect(rect[0], rect[1], point) as i8
        + do_intersect(rect[1], rect[2], point) as i8
        + do_intersect(rect[2], rect[3], point) as i8
        + do_intersect(rect[3], rect[0], point) as i8;

    counter % 2 == 1
}

pub fn rotate_point(start: Point, origin: Point, degrees: f32) -> Point {
    let deg2_rad = 3.14159 / 180.0;

    let radians = degrees * deg2_rad;

    let mut point = start;

    // translate point back to origin:
    point.x -= origin.x;
    point.y -= origin.y;

    // rotate point
    let xnew: i32 = ((point.x as f32 * radians.cos()) - (point.y as f32 * radians.sin())) as i32;
    let ynew: i32 = ((point.x as f32 * radians.sin()) + (point.y as f32 * radians.cos())) as i32;

    // translate point back:
    point.x = xnew + origin.x;
    point.y = ynew + origin.y;

    point
}

pub fn rotate_rectangle(active_rect: sdl2::rect::Rect, rotation: f32, scale: f32) -> [Point; 4] {
    let temp_center = active_rect.top_left()
        + Point::new(
            (active_rect.width() as f32 * scale / 2.0) as i32,
            (active_rect.height() as f32 * scale / 2.0) as i32,
        );

    let top_left = rotate_point(active_rect.top_left(), temp_center, rotation);
    let top_right = rotate_point(
        active_rect.top_left() + Point::new((active_rect.width() as f32 * scale) as i32, 0),
        temp_center,
        rotation,
    );
    let bottom_left = rotate_point(
        active_rect.top_left() + Point::new(0, (active_rect.height() as f32 * scale) as i32),
        temp_center,
        rotation,
    );
    let bottom_right = rotate_point(
        active_rect.top_left()
            + Point::new(
                (active_rect.width() as f32 * scale) as i32,
                (active_rect.height() as f32 * scale) as i32,
            ),
        temp_center,
        rotation,
    );

    [top_left, top_right, bottom_right, bottom_left] //order is very important]
}
