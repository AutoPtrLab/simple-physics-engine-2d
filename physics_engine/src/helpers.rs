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
        Shape::Rectangle {
            half_width,
            half_height,
        } => get_rect_points(pos, half_width, half_height),
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
pub fn get_rect_points<const N: usize>(pos: Vec2, half_w: f32, half_h: f32) -> [Vec2; N] {
    const { assert!(N >= 4, "THE MINIMUM POINTS TO DESCRIBE A Rectangle IS 4") };
    let w = half_w * 2.0;
    let h = half_h * 2.0;
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
    const {
        assert!(
            N >= 4,
            "THE MINIMUM POINTS TO DESCRIBE A LINE is 4 (if you want the points , the line body already has it"
        )
    }
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

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;
    use crate::v2;

    #[test]
    fn test_get_circle_points() {
        let points = get_circle_points::<4>(v2!(0.0, 0.0), 10.0);
        assert_eq!(points.len(), 4);
        assert!(points[0].x > 9.9 && points[0].y.abs() < 0.1);
    }

    #[test]
    fn test_get_rect_points() {
        let points = get_rect_points::<4>(v2!(0.0, 0.0), 5.0, 10.0);
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], v2!(-5.0, -10.0));
        assert_eq!(points[1], v2!(5.0, -10.0));
        assert_eq!(points[2], v2!(-5.0, 10.0));
        assert_eq!(points[3], v2!(-5.0, -10.0));
    }

    #[test]
    fn test_get_rect_points_asymmetric() {
        let points = get_rect_points::<6>(v2!(0.0, 0.0), 5.0, 5.0);
        assert_eq!(points.len(), 6);
    }

    #[test]
    fn test_get_capsule_points() {
        let points = get_capsule_points::<8>(v2!(0.0, 0.0), 10.0, 5.0, 0.0);
        assert_eq!(points.len(), 8);
    }

    #[test]
    fn test_get_shape_points_circle() {
        let shape = Shape::Circle { rad: 5.0 };
        let points = get_shape_points::<4>(v2!(0.0, 0.0), shape, None);
        assert_eq!(points.len(), 4);
    }

    #[test]
    fn test_get_shape_points_rect() {
        let shape = Shape::Rectangle {
            half_width: 5.0,
            half_height: 5.0,
        };
        let points = get_shape_points::<4>(v2!(0.0, 0.0), shape, None);
        assert_eq!(points.len(), 4);
    }
    #[test]
    fn test_get_line_points() {
        // Usamos 12.0 en lugar de 10.0 para que al dividir en 3 segmentos dé un número exacto (4.0)
        let points = get_line_points::<4>(v2!(0.0, 0.0), v2!(12.0, 0.0));

        assert_eq!(points.len(), 4);
        assert_eq!(points[0], v2!(0.0, 0.0));
        assert_eq!(points[1], v2!(4.0, 0.0));
        assert_eq!(points[2], v2!(8.0, 0.0));
        assert_eq!(points[3], v2!(12.0, 0.0));
    }

    #[test]
    fn test_get_shape_points_capsule() {
        let shape = Shape::Capsule {
            rad: 5.0,
            half_len: 10.0,
        };
        let points = get_shape_points::<4>(v2!(0.0, 0.0), shape, Some(0.0));
        assert_eq!(points.len(), 4);
    }

    #[test]
    #[should_panic(expected = "The capsule needs to know the angle")]
    fn test_get_shape_points_capsule_panics_without_angle() {
        let shape = Shape::Capsule {
            rad: 5.0,
            half_len: 10.0,
        };
        get_shape_points::<4>(v2!(0.0, 0.0), shape, None);
    }
}
