use crate::v2;
use crate::world;
use crate::{
    body::{Body, Shape},
    math::Vec2,
};

pub fn update_collisions(w: &mut world::World) {
    for i in 0..w.bodies.len() {
        for j in (i + 1)..w.bodies.len() {
            let (a, b) = w.bodies.split_at_mut(j);
            check_collision(&mut a[i], &mut b[0]);
        }
    }
}

fn check_collision(a: &mut Body, b: &mut Body) {
    let info = match (a.shape, b.shape) {
        // --- CIRCLE VS ALL ---
        (Shape::Circle { rad: rad_a }, Shape::Circle { rad: rad_b }) => {
            // Your Circle vs Circle function
            collision_circle_circle(a.pos, rad_a, b.pos, rad_b)
        }
        (Shape::Circle { rad }, Shape::Rectangle { width, height }) => {
            collision_circle_rect(a.pos, rad, b.pos, width, height)
        }
        (Shape::Circle { rad }, Shape::Line { p }) => {
            // Your Circle vs Line function
            todo!();
        }

        // --- RECTANGLE VS ALL ---
        (
            Shape::Rectangle {
                width: w_a,
                height: h_a,
            },
            Shape::Rectangle {
                width: w_b,
                height: h_b,
            },
        ) => {
            // Your Rectangle vs Rectangle function
            collision_rect_rect(a.pos, w_a, h_a, b.pos, w_b, h_b)
        }
        (Shape::Rectangle { width, height }, Shape::Circle { rad }) => {
            //  Rectangle vs Circle function , need to be inverted
            collision_circle_rect(b.pos, rad, a.pos, width, height).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
            })
        }

        (Shape::Rectangle { width, height }, Shape::Line { p }) => {
            // Your Rectangle vs Line function
            todo!()
        }

        // --- LINE VS ALL ---
        (Shape::Line { p: p1_a }, Shape::Line { p: p1_b }) => {
            // Your Line vs Line function
            todo!()
        }
        (Shape::Line { p }, Shape::Circle { rad }) => {
            // Your Line vs Circle function (you can call the inverse)
            todo!()
        }
        (Shape::Line { p }, Shape::Rectangle { width, height }) => {
            // Your Line vs Rectangle function (you can call the inverse)
            todo!()
        }
    };
    //corrections and impulse calc
    if let Some(info) = info {
        let inv_mass_tot = a.inv_mass + b.inv_mass;
        if inv_mass_tot <= 0.0 {
            //two static bodies
            return;
        }
        let imp = (((a.vel - b.vel).dot(info.n)) * -2.0) / inv_mass_tot; //calculation of the impulse
        a.vel += info.n * imp * a.inv_mass;
        b.vel -= info.n * imp * b.inv_mass;

        //the momvent rate represent how much of the depth is affected to each body
        let movement_rate_a = a.inv_mass / inv_mass_tot;
        let movement_rate_b = b.inv_mass / inv_mass_tot;

        //each body only gets affected of its proportional depth taking the heavier the object the less change
        a.pos += info.depth * info.n * movement_rate_a;
        b.pos -= info.depth * info.n * movement_rate_b;
    }
}

// ------------------------------------------
// COLLIDER FUNCTIONS FUNCTIONS
// ------------------------------------------

struct CollisionInfo {
    n: Vec2,    //Perpendicular vector to the surface
    depth: f32, //distance the bodies has entered each other in the n vector direction
}
fn collision_circle_rect(circle_pos: Vec2, rad: f32, rect_pos: Vec2, width: f32, height: f32) -> Option<CollisionInfo> {
    // clamping the nearest point of the rect to the circle
    let rect_left = rect_pos.x - width * 0.5;
    let rect_top = rect_pos.y - height * 0.5;

    //we look for the closes coord in the rect to the circle
    let closest_x = circle_pos.x.clamp(rect_left, rect_left + width);
    let closest_y = circle_pos.y.clamp(rect_top, rect_top + height);

    let dx = circle_pos.x - closest_x;
    let dy = circle_pos.y - closest_y;

    if (dx * dx + dy * dy) > (rad * rad) {
        return None;
    }; //if they dont collide this func does not return nothing

    let ab_vec = circle_pos - v2![closest_x, closest_y]; //vector representing the distance between the center of the circle and the nearer side of the rect
    let dist = ab_vec.len();
    if dist == 0.0 {
        //if the are so close they are in the same coord, limit case
        return Some(CollisionInfo {
            n: v2![0.0, 1.0],
            depth: rad,
        });
    }
    Some(CollisionInfo {
        n: ab_vec.normalize(),
        depth: (rad - dist),
    })
}
fn collision_circle_circle(a_pos: Vec2, ra: f32, b_pos: Vec2, rb: f32) -> Option<CollisionInfo> {
    let ab_vec = a_pos - b_pos;
    let min_dist_sq = ab_vec.len_sq();
    //if the distance is bigger is not colliding
    if min_dist_sq > ((ra + rb) * (ra + rb)) {
        return None;
    }

    let dist = ab_vec.len();
    let ideal_dist = ra + rb;
    Some(CollisionInfo {
        n: ab_vec.normalize(),
        depth: ideal_dist - dist,
    })
}
//only supporting AABB
fn collision_rect_rect(a_pos: Vec2, w_a: f32, h_a: f32, b_pos: Vec2, w_b: f32, h_b: f32) -> Option<CollisionInfo> {
    let x_a = a_pos.x - w_a * 0.5;
    let y_a = a_pos.y - h_a * 0.5;
    let x_b = b_pos.x - w_b * 0.5;
    let y_b = b_pos.y - h_b * 0.5;
    //AABB
    if !(x_a < x_b + w_b && x_a + w_a > x_b && y_a < y_b + h_b && y_a + h_a > y_b) {
        return None;
    }
    //to find the depth we have to determine which faces are facing eachother
    // this is the distance that have between the two centers
    let marginal_dist_x = (w_a + w_b) * 0.5;
    let marginal_dist_y = (h_a + h_b) * 0.5;

    let dx = a_pos.x - b_pos.x;
    let dy = a_pos.y - b_pos.y;

    let depth_x = marginal_dist_x - dx.abs();
    let depth_y = marginal_dist_y - dy.abs();
    //since we now ther is a collision

    if depth_x < depth_y {
        //side collision
        let n_x = if dx > 0.0 { 1.0 } else { -1.0 }; //if dx is neg they collide in the left side

        Some(CollisionInfo {
            n: v2!(n_x, 0.0),
            depth: depth_x,
        })
    } else {
        //top down collision
        let n_y = if dy > 0.0 { 1.0 } else { -1.0 };
        Some(CollisionInfo {
            n: v2!(0.0, n_y),
            depth: depth_y,
        })
    }
}
