use crate::body_shapes::body::Shape;
/// this file holds the helper if you need  to now if a body contains another body . they just return a boolean if the body contains the other one, if
/// the container is contained in the  body is going to be false, is onlyb true if the container body contains the other body
use crate::math::Vec2;
use crate::v2;

//circle that contains another circle, checks if b is contained in a(NOT in reverse)
pub fn circ_contains_circ(a_pos: Vec2, a_rad: f32, b_pos: Vec2, b_rad: f32) -> bool {
    // if the b circle is bigger is imposible to be contain in a
    if a_rad < b_rad {
        return false;
    }
    let dist = (a_pos - b_pos).len();
    dist <= a_rad - b_rad
}
///if the circle contains the rectangle ( the rect position is in the center of the rectangle)
pub fn circ_contains_rect(circ_pos: Vec2, rad: f32, rect_pos: Vec2, rect_w: f32, rect_h: f32) -> bool {
    let half_w: f32 = rect_w / 2.0;
    let half_h: f32 = rect_h / 2.0;

    let left_up_corner = v2!(rect_pos.x - half_w, rect_pos.y - half_h);
    let left_down_corner = v2!(rect_pos.x - half_w, rect_pos.y + half_h);
    let right_up_corner = v2!(rect_pos.x + half_w, rect_pos.y - half_h);
    let right_down_corner = v2!(rect_pos.x + half_w, rect_pos.y + half_h);

    if !circ_contains_point(circ_pos, rad, left_up_corner) {
        return false;
    }
    if !circ_contains_point(circ_pos, rad, left_down_corner) {
        return false;
    }
    if !circ_contains_point(circ_pos, rad, right_up_corner) {
        return false;
    }
    if !circ_contains_point(circ_pos, rad, right_down_corner) {
        return false;
    }
    //if one of the corners is not in the circle return false else true

    true
}

///Check if a point is contained in a circle
pub fn circ_contains_point(circ_pos: Vec2, rad: f32, point: Vec2) -> bool {
    let dist_sq = (circ_pos - point).len_sq();

    dist_sq < (rad * rad)
}
/// checks if the a  contains  the b rect (only AABB rectagles)
pub fn rect_contains_rect(a_pos: Vec2, a_w: f32, a_h: f32, b_pos: Vec2, b_w: f32, b_h: f32) -> bool {
    let b_half_w = b_w * 0.5;
    let b_half_h = b_h * 0.5;
    let left_up_corner = v2!(b_pos.x - b_half_w, b_pos.y - b_half_h);
    let left_down_corner = v2!(b_pos.x - b_half_w, b_pos.y + b_half_h);
    let right_up_corner = v2!(b_pos.x + b_half_w, b_pos.y - b_half_h);
    let right_down_corner = v2!(b_pos.x + b_half_w, b_pos.y + b_half_h);

    if !rect_contains_point(a_pos, a_w, a_h, left_up_corner) {
        return false;
    }
    if !rect_contains_point(a_pos, a_w, a_h, left_down_corner) {
        return false;
    }
    if !rect_contains_point(a_pos, a_w, a_h, right_up_corner) {
        return false;
    }
    if !rect_contains_point(a_pos, a_w, a_h, right_down_corner) {
        return false;
    }
    true
}
///checks if a point is contained in the rect
pub fn rect_contains_point(rect_pos: Vec2, width: f32, height: f32, point: Vec2) -> bool {
    let half_w: f32 = width / 2.0;
    let half_h: f32 = height / 2.0;
    (point.x >= (rect_pos.x - half_w))
        && (point.x <= (rect_pos.x + half_w))
        && (point.y >= rect_pos.y - half_h)
        && (point.y <= (rect_pos.y + half_h))
}
/// checks if the rect contains a circle
pub fn rect_contains_circle(rect_pos: Vec2, width: f32, height: f32, circ_pos: Vec2, rad: f32) -> bool {
    let top_left = v2!(rect_pos.x - width / 2.0, rect_pos.y - height / 2.0);

    top_left.x < circ_pos.x - rad
        && top_left.x + width > circ_pos.x + rad
        && top_left.y < circ_pos.y - rad
        && top_left.y + height > circ_pos.y + rad
}

pub fn capsule_contains_point(cap_pos: Vec2, half_len: f32, rad: f32, ang: f32, point: Vec2) -> bool {
    //try to find the nearest point in the capsule line

    let cap_point_vec = point - cap_pos;
    let line_vec = v2!(ang.cos(), ang.sin());
    let proy = cap_point_vec.dot(line_vec);

    if proy.abs() > half_len {
        return false;
    }
    let nearest_line_point = cap_pos + line_vec * proy;

    circ_contains_point(nearest_line_point, rad, point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec2;
    use crate::v2;

    #[test]
    fn test_circ_contains_circ() {
        assert!(circ_contains_circ(v2!(0.0, 0.0), 10.0, v2!(2.0, 2.0), 3.0));
        assert!(!circ_contains_circ(v2!(0.0, 0.0), 5.0, v2!(2.0, 2.0), 10.0));
        assert!(!circ_contains_circ(v2!(0.0, 0.0), 5.0, v2!(20.0, 20.0), 2.0));
    }

    #[test]
    fn test_circ_contains_rect() {
        assert!(circ_contains_rect(v2!(0.0, 0.0), 10.0, v2!(0.0, 0.0), 4.0, 4.0));
        assert!(!circ_contains_rect(v2!(0.0, 0.0), 5.0, v2!(0.0, 0.0), 20.0, 20.0));
        assert!(!circ_contains_rect(v2!(0.0, 0.0), 10.0, v2!(15.0, 15.0), 2.0, 2.0));
    }

    #[test]
    fn test_circ_contains_point() {
        assert!(circ_contains_point(v2!(0.0, 0.0), 5.0, v2!(2.0, 2.0)));
        assert!(!circ_contains_point(v2!(0.0, 0.0), 5.0, v2!(10.0, 10.0)));
    }

    #[test]
    fn test_rect_contains_rect() {
        assert!(rect_contains_rect(v2!(0.0, 0.0), 20.0, 20.0, v2!(0.0, 0.0), 10.0, 10.0));
        assert!(!rect_contains_rect(
            v2!(0.0, 0.0),
            10.0,
            10.0,
            v2!(0.0, 0.0),
            20.0,
            20.0
        ));
        assert!(!rect_contains_rect(
            v2!(0.0, 0.0),
            20.0,
            20.0,
            v2!(30.0, 30.0),
            5.0,
            5.0
        ));
    }

    #[test]
    fn test_rect_contains_point() {
        assert!(rect_contains_point(v2!(0.0, 0.0), 10.0, 10.0, v2!(3.0, 3.0)));
        assert!(!rect_contains_point(v2!(0.0, 0.0), 10.0, 10.0, v2!(15.0, 15.0)));
    }

    #[test]
    fn test_rect_contains_circle() {
        assert!(rect_contains_circle(v2!(0.0, 0.0), 20.0, 20.0, v2!(0.0, 0.0), 5.0));
        assert!(!rect_contains_circle(v2!(0.0, 0.0), 10.0, 10.0, v2!(0.0, 0.0), 20.0));
        assert!(!rect_contains_circle(v2!(0.0, 0.0), 20.0, 20.0, v2!(30.0, 30.0), 2.0));
    }

    #[test]
    fn test_capsule_contains_point() {
        assert!(capsule_contains_point(v2!(0.0, 0.0), 10.0, 5.0, 0.0, v2!(5.0, 2.0)));
        assert!(!capsule_contains_point(v2!(0.0, 0.0), 10.0, 5.0, 0.0, v2!(15.0, 2.0)));
        assert!(!capsule_contains_point(v2!(0.0, 0.0), 10.0, 5.0, 0.0, v2!(5.0, 10.0)));
    }
}
