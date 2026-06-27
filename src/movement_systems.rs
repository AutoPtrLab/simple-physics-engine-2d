use crate::body::Body;
pub fn update_movement(body_vec: &mut Vec<Body>, dt: f32) {
    for b in body_vec {
        b.vel += b.accel * dt;
        b.pos += b.vel * dt;
    }
}
