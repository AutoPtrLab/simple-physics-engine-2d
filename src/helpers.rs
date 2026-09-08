///This points functions can be use to assert the overlapping with more complex shapes
use std::f32::consts::{PI, TAU};
use std::io::LineWriter;

use crate::body_shapes::body::Shape;
use crate::math::Vec2;
use crate::v2;

///return the <N> points of the shape, this function only uses ang in the capsule match, if you know the shape you are comparing
/// try using the particular functions
pub fn get_shape_points<const N: usize>(pos: Vec2, shape: Shape, ang: Option<f32>) -> [Vec2; N] {
    match shape {
        Shape::Circle { rad } => get_circle_points(pos, rad),
        Shape::Rectangle { width, height } => get_rect_points(pos, width, height),
        Shape::Line { p } => get_line_points(pos, p),
        Shape::Capsule { rad, half_len } => get_capsule_points(
            pos,
            half_len,
            rad,
            ang.expect("The capsule needs to know the angle in get_shape_points"),
        ),
    }
}

///returns the N points of a circle
pub fn get_circle_points<const N: usize>(pos: Vec2, rad: f32) -> [Vec2; N] {
    //how each point is from each other depending on the N constant
    const { assert!(N >= 4, "THE MINIMUM POINTS TO DESCRIBE A Circle IS 4") };
    let ang_resolution = const { TAU / N as f32 };
    std::array::from_fn(|i| {
        let ang = ang_resolution * i as f32;
        Vec2 {
            x: pos.x + rad * ang.cos(),
            y: pos.y + rad * ang.sin(),
        }
    })
}
///this functinos need at least N to be 4
pub fn get_rect_points<const N: usize>(pos: Vec2, w: f32, h: f32) -> [Vec2; N] {
    let half_w = w / 2.0;
    let half_h = h / 2.0;
    const { assert!(N >= 4, "THE MINIMUM POINTS TO DESCRIBE A Rectangle IS 4") };

    let n = const { N / 4 };
    let mut particions = [n; 4];

    for i in 0..(N - (n * 4)) {
        particions[i % 4] += 1;
    }
    let mut points = [pos; N];

    for i in 0..particions[0] {
        points[i] = v2!((pos.x - half_w) + (w / particions[0] as f32) * i as f32, pos.y - half_h);
    }
    for i in 0..particions[1] {
        points[i + particions[0]] = v2!(pos.x + half_w, (pos.y - half_h) + (h / particions[1] as f32) * i as f32);
    }
    for i in 0..particions[2] {
        points[i + particions[0] + particions[1]] =
            v2!((pos.x - half_w) + (w / particions[2] as f32) * i as f32, pos.y + half_h);
    }
    for i in 0..particions[3] {
        points[i + particions[0] + particions[1] + particions[2]] =
            v2!(pos.x - half_w, (pos.y - half_h) + (h / particions[3] as f32) * i as f32);
    }

    points
}

pub fn get_line_points<const N: usize>(p1: Vec2, p2: Vec2) -> [Vec2; N] {
    const { assert!(N >= 2, "THE MINIMUM POINTS TO DESCRIBE A LINE is 2") }
    let len = (p1 - p2).len();
    let resolution = len / (N - 1) as f32;
    let n = (p2 - p1).normalize();
    std::array::from_fn(|i| p1 + n * resolution * i as f32)
}

/// return N points describinc the perimeter of a capsule
pub fn get_capsule_points<const N: usize>(pos: Vec2, half_len: f32, rad: f32, ang: f32) -> [Vec2; N] {
    const { assert!(N >= 4, "THE MINIMUM POINTS TO DESCRIBE A Capsule IS 4") };
    //the unit vector of the  line describing the capsule
    let line_n = v2!(ang.cos(), ang.sin());
    //the unit vector of the perpendicular to the line _n
    let perp_n = v2!(ang.sin(), ang.cos());

    let n = const { N / 4 };
    let mut particions = [n; 4];

    for i in 0..(N - (n * 4)) {
        particions[i % 4] += 1;
    }
    let mut points = [pos; N];

    //amgle resolution on the first circle
    let a1 = PI / particions[0] as f32;
    let a2 = PI / particions[2] as f32;
    let l1 = (half_len * 2.0) / particions[1] as f32;
    let l2 = (half_len * 2.0) / particions[3] as f32;
    //first half circle
    for i in 0..particions[0] {
        points[i] = (pos - line_n * half_len) + v2!(rad * (a1 * i as f32).cos(), rad * (a1 * i as f32).sin());
    }
    //upper line
    for i in 0..particions[1] {
        points[i + particions[0]] = (pos - line_n * half_len - perp_n * rad) + line_n * l1 * i as f32;
    }
    //second half circle
    for i in 0..particions[2] {
        points[i + particions[0] + particions[1]] =
            (pos + line_n * half_len) + v2!(rad * (a2 * i as f32).cos(), -rad * (a2 * i as f32).sin());
    }
    //down line
    for i in 0..particions[3] {
        points[i + particions[0] + particions[1] + particions[2]] =
            (pos + line_n * half_len + perp_n * rad) - line_n * l2 * i as f32;
    }

    points
}
