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
        true
    } else {
        false
    }
}

pub fn check_rect(rect: &sdl2::rect::Rect, point: Point) -> bool {
    let counter: i8 = do_intersect(rect.top_left(), rect.top_right(), point) as i8
        + do_intersect(rect.top_right(), rect.bottom_right(), point) as i8
        + do_intersect(rect.bottom_right(), rect.bottom_left(), point) as i8
        + do_intersect(rect.bottom_left(), rect.top_left(), point) as i8;

    counter % 2 == 1
}
