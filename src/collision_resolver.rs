use crate::body;
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
            let info = check_collision(&a[i], &b[0]);
            apply_forces(&mut a[i], &mut b[0], info);
        }
    }

    for _ in 0..4 {
        for i in 0..w.bodies.len() {
            for j in (i + 1)..w.bodies.len() {
                let (a, b) = w.bodies.split_at_mut(j);
                let info = check_collision(&a[i], &b[0]);
                resolve_collision(&mut a[i], &mut b[0], info);
                //resolve_collision_2(&mut a[i], &mut b[0], info);
            }
        }
    }
}

fn check_collision(a: &Body, b: &Body) -> Option<CollisionInfo> {
    match (a.shape, b.shape) {
        // --- CIRCLE VS ALL ---
        (Shape::Circle { rad: rad_a }, Shape::Circle { rad: rad_b }) =>
        // Your Circle vs Circle function
        {
            collision_circle_circle(a.pos, rad_a, b.pos, rad_b)
        }

        (Shape::Circle { rad }, Shape::Rectangle { width, height }) => {
            collision_circle_rect(a.pos, rad, b.pos, width, height)
        }

        (Shape::Circle { rad }, Shape::Line { p }) => collision_circle_line(a.pos, rad, b.pos, p),
        //circle vs capsule
        (Shape::Circle { rad: circ_rad }, Shape::Capsule { rad: cap_rad, half_len }) => {
            collision_circle_capsule(a.pos, circ_rad, b.pos, cap_rad, half_len, b.ang)
        }
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
                impact_point: info.impact_point,
            })
        }

        (Shape::Rectangle { width, height }, Shape::Line { p }) => collision_rect_line(a.pos, width, height, b.pos, p),
        (Shape::Rectangle { width, height }, Shape::Capsule { rad, half_len }) => {
            collision_rect_capsule(a.pos, width, height, b.pos, rad, half_len, b.ang).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
                impact_point: info.impact_point,
            })
        }
        // --- LINE VS ALL ---
        (Shape::Line { p: _p1_a }, Shape::Line { p: _p1_b }) => None,
        (Shape::Line { p }, Shape::Circle { rad }) => {
            collision_circle_line(b.pos, rad, a.pos, p).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
                impact_point: info.impact_point,
            })
        }
        (Shape::Line { p }, Shape::Rectangle { width, height }) => {
            // Your Line vs Rectangle function (you can call the inverse)
            collision_rect_line(b.pos, width, height, a.pos, p).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
                impact_point: info.impact_point,
            })
        }
        (Shape::Line { p: line_p }, Shape::Capsule { rad, half_len }) => {
            collision_line_capsule(a.pos, line_p, b.pos, half_len, rad, b.ang).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
                impact_point: info.impact_point,
            })
        }
        (Shape::Capsule { rad: cap_rad, half_len }, Shape::Circle { rad: circ_rad }) => {
            collision_circle_capsule(b.pos, circ_rad, a.pos, cap_rad, half_len, a.ang).map(|info| CollisionInfo {
                n: -info.n,
                depth: info.depth,
                impact_point: info.impact_point,
            })
        }
        (Shape::Capsule { rad, half_len }, Shape::Line { p: line_p }) => {
            collision_line_capsule(b.pos, line_p, a.pos, half_len, rad, a.ang)
        }
        (Shape::Capsule { rad, half_len }, Shape::Rectangle { width, height }) => {
            collision_rect_capsule(b.pos, width, height, a.pos, rad, half_len, a.ang)
        }
        (
            Shape::Capsule {
                rad: rad_a,
                half_len: half_len_a,
            },
            Shape::Capsule {
                rad: rad_b,
                half_len: half_len_b,
            },
        ) => collision_capsule_capsule(a.pos, rad_a, half_len_a, a.ang, b.pos, rad_b, half_len_b, b.ang),
    }
}
fn resolve_collision(body_a: &mut Body, body_b: &mut Body, info: Option<CollisionInfo>) {
    if let Some(info) = info {
        let inv_mass_tot = body_a.inv_mass + body_b.inv_mass;
        if inv_mass_tot == 0.0 {
            //two static objects
            body_a.pos += info.depth * info.n * 0.5;
            body_b.pos -= info.depth * info.n * 0.5;
            return;
        }
        //the momvent rate represent how much of the depth is affected to each body
        let movement_rate_a = body_a.inv_mass / inv_mass_tot;
        let movement_rate_b = body_b.inv_mass / inv_mass_tot;
        //each body only gets affected of its proportional depth taking the heavier the object the less change
        body_a.pos += info.depth * info.n * movement_rate_a;
        body_b.pos -= info.depth * info.n * movement_rate_b;
    }
}

fn apply_forces(body_a: &mut Body, body_b: &mut Body, info: Option<CollisionInfo>) {
    if let Some(info) = info {
        let inv_mass_tot = body_a.inv_mass + body_b.inv_mass;
        if inv_mass_tot <= 0.0 {
            match (body_a.shape, body_b.shape) {
                (Shape::Rectangle { .. }, Shape::Rectangle { .. }) => {
                    body_a.vel = -body_a.vel;
                    body_b.vel = -body_b.vel;
                    return;
                }
                (Shape::Line { .. }, Shape::Rectangle { .. }) => {
                    body_b.vel = -body_b.vel;
                    return;
                }
                (Shape::Rectangle { .. }, Shape::Line { .. }) => {
                    body_a.vel = -body_a.vel;
                    return;
                }
                _ => (),
            }
        }
        let collision_point = info
            .impact_point
            .expect("After the Line vs Rect and Rect vs Rect there cannot be a None in the collision point");
        let rel_vel = body_a.vel - body_b.vel; //relative velocity of a respect to b
        let vel_along_normal = rel_vel.dot(info.n); //rel vel affecting the normal vector
        if vel_along_normal <= 0.0 {
            //if a is getting nearer to b we apply the impulse
            let imp = (vel_along_normal * -2.0) / inv_mass_tot;
            body_a.vel += info.n * imp * body_a.inv_mass;
            body_b.vel -= info.n * imp * body_b.inv_mass;
            body_a.ang_vel += ((collision_point - body_a.pos).cross(info.n * imp)) * body_a.inv_inert;
            body_b.ang_vel += ((collision_point - body_b.pos).cross(-(info.n * imp))) * body_b.inv_inert;
        }
    }
}

// ------------------------------------------
// COLLIDER FUNCTIONS FUNCTIONS
// ------------------------------------------
#[derive(Clone, Copy)]
struct CollisionInfo {
    n: Vec2,                    //Perpendicular vector to the surface
    depth: f32,                 //distance the bodies has entered each other in the n vector direction
    impact_point: Option<Vec2>, //point where the two bodies collided if is none is because there is no rotation
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
            impact_point: Some(v2![0.0, 1.0] * rad + circle_pos),
        });
    }
    let n = ab_vec.normalize();
    Some(CollisionInfo {
        n,
        depth: (rad - dist),
        impact_point: Some(n * rad + circle_pos),
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
    let n = ab_vec.normalize();
    Some(CollisionInfo {
        n,
        depth: ideal_dist - dist,
        impact_point: Some(n * ra + a_pos),
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
            impact_point: None,
        })
    } else {
        //top down collision
        let n_y = if dy > 0.0 { 1.0 } else { -1.0 };
        Some(CollisionInfo {
            n: v2!(0.0, n_y),
            depth: depth_y,
            impact_point: None,
        })
    }
}
fn collision_rect_line(rect_pos: Vec2, w: f32, h: f32, p1_pos: Vec2, p2_pos: Vec2) -> Option<CollisionInfo> {
    let rx = w / 2.0;
    let ry = h / 2.0;

    let line_min_x = p1_pos.x.min(p2_pos.x);
    let line_max_x = p1_pos.x.max(p2_pos.x);
    let line_min_y = p1_pos.y.min(p2_pos.y);
    let line_max_y = p1_pos.y.max(p2_pos.y);

    if rect_pos.x + rx <= line_min_x || rect_pos.x - rx >= line_max_x {
        return None;
    }
    if rect_pos.y + ry <= line_min_y || rect_pos.y - ry >= line_max_y {
        return None;
    }

    let line_vec = Vec2 {
        x: p2_pos.x - p1_pos.x,
        y: p2_pos.y - p1_pos.y,
    };
    let len = line_vec.len();

    let dir = Vec2 {
        x: line_vec.x / len,
        y: line_vec.y / len,
    };
    let mut normal = Vec2 { x: -dir.y, y: dir.x }; //vector normal to the line

    let vector_to_rect = Vec2 {
        x: rect_pos.x - p1_pos.x,
        y: rect_pos.y - p1_pos.y,
    };

    let proj_dir = vector_to_rect.dot(dir);
    let r_proj_dir = rx * dir.x.abs() + ry * dir.y.abs();

    if proj_dir + r_proj_dir <= 0.0 || proj_dir - r_proj_dir >= len {
        return None;
    }

    let mut dist_to_line = vector_to_rect.dot(normal);

    if dist_to_line < 0.0 {
        normal = Vec2 {
            x: -normal.x,
            y: -normal.y,
        };
        dist_to_line = -dist_to_line;
    }

    let r_proj_normal = rx * normal.x.abs() + ry * normal.y.abs();
    let depth = r_proj_normal - dist_to_line;
    if depth <= 0.0 {
        return None;
    }

    Some(CollisionInfo {
        n: normal,
        depth,
        impact_point: None,
    })
}
///Resolver between circles and lines
fn collision_circle_line(circle_pos: Vec2, rad: f32, p1_pos: Vec2, p2_pos: Vec2) -> Option<CollisionInfo> {
    //maybe sq is more efficient and the n vector can be precalculate
    let line_vec = p2_pos - p1_pos;

    let p1_circ_vec = circle_pos - p1_pos;

    let projection_lenght = p1_circ_vec.dot(line_vec) / line_vec.len();
    let nearest_point = p1_pos + line_vec.normalize() * projection_lenght;
    let real_x = nearest_point.x.clamp(p1_pos.x.min(p2_pos.x), p1_pos.x.max(p2_pos.x));
    let real_y = nearest_point.y.clamp(p1_pos.y.min(p2_pos.y), p1_pos.y.max(p2_pos.y));

    let dist_vec = circle_pos - v2!(real_x, real_y);

    //if the dist is bigger than the radius they are not colliding
    if dist_vec.len_sq() > (rad * rad) {
        //println!("{}", dist);
        return None;
    };
    // println!("{}", collision_vec.normalize());
    let n = dist_vec.normalize();
    Some(CollisionInfo {
        n,
        depth: rad - dist_vec.len(),
        impact_point: Some(n * rad + circle_pos),
    })
}
fn collision_circle_capsule(
    circle_pos: Vec2,
    circ_rad: f32,
    cap_pos: Vec2,
    cap_rad: f32,
    cap_hl: f32, //capsule half legth
    cap_ang: f32,
) -> Option<CollisionInfo> {
    let offset_dir = v2!(cap_ang.cos(), cap_ang.sin());
    let circ_cap_vec = circle_pos - cap_pos; //vector capsule(center) - circle

    let pro_len = circ_cap_vec.dot(offset_dir).clamp(-cap_hl, cap_hl); //the lenght of the projection, can be negative if its in the other direction respect to certer_p1:Vec

    let nearest_point = cap_pos + pro_len * offset_dir;

    let vec = circle_pos - nearest_point; //vector from the nearest point of the capsule to the circle

    if vec.len_sq() > (circ_rad + cap_rad) * (circ_rad + cap_rad) {
        //if the distance is bigger than the two radius is to far
        return None;
    }
    let distance = vec.len(); //real distance
    //println!("{}", vec.normalize());
    let n = vec.normalize();
    Some(CollisionInfo {
        n,
        depth: (circ_rad + cap_rad) - distance,
        impact_point: Some(n * circ_rad + circle_pos),
    })
}
//The rect cant rotate
fn collision_rect_capsule(
    rect_pos: Vec2,
    w: f32,
    h: f32,
    cap_pos: Vec2,
    rad: f32,
    cap_hl: f32,
    cap_ang: f32,
) -> Option<CollisionInfo> {
    //Calculation of the nearest poiint of hte capssule in the circle
    let cap_rect_vec = rect_pos - cap_pos;
    let capsule_line = v2!(cap_ang.cos(), cap_ang.sin());

    let proj_len = cap_rect_vec.dot(capsule_line).clamp(-cap_hl, cap_hl); //projection

    let circle_pos = proj_len * capsule_line + cap_pos; //point representing the subcircle insid ef thee capsule
    //Since now we are working with a circle and a rectangle we can call our circle vs rect collider resoveler
    collision_circle_rect(circle_pos, rad, rect_pos, w, h)
}
fn collision_line_capsule(
    line_p1: Vec2,
    line_p2: Vec2,
    cap_pos: Vec2,
    cap_hl: f32,

    rad: f32,
    cap_ang: f32,
) -> Option<CollisionInfo> {
    let line_vec = line_p2 - line_p1;
    //let line_dir = line_vec.normalize(); //direction vector of the line
    let cap_dir = v2!(cap_ang.cos(), cap_ang.sin()); // direction vector of the capsule
    let cap_p1 = cap_pos + cap_dir * cap_hl;
    let cap_p2 = cap_pos - cap_dir * cap_hl;
    let cap_vec = cap_p2 - cap_p1;
    let r = line_p1 - cap_p1;

    // variables to find the nearest point of each line
    let a = cap_vec.len_sq();
    let e = line_vec.len_sq();
    let f = line_vec.dot(r);
    let c = cap_vec.dot(r);
    let b = cap_vec.dot(line_vec);

    //parametrics variables
    let mut s: f32;

    //s = (b*f -c*e)/(a*e-b²) -> vectorial equation to find the parametrical that shows as the nearest point
    let denom = a * e - b * b;
    if denom != 0.0 {
        //if the dot product is zero the lines are paralel
        s = ((b * f - c * e) / denom).clamp(0.0, 1.0); //clamping because we are dealing with segments and not infinite lines
    } else {
        //the two line are paralel so we just pick the first point of the capsule
        s = 0.0;
    }
    //parametrical value
    let t = (b * s + f) / e;

    //t edge case( when the nearest point is not in the capsule segment)
    // we have to also recalculate s since we have a new t
    if t < 0.0 {
        s = (-c / a).clamp(0.0, 1.0);
    } else if t > 1.0 {
        s = ((b - c) / a).clamp(0.0, 1.0);
    }

    let virtual_circ = cap_p1 + s * cap_dir;
    collision_circle_line(virtual_circ, rad, line_p1, line_p2)
}
fn collision_capsule_capsule(
    pos_a: Vec2,
    rad_a: f32,
    hl_a: f32,
    ang_a: f32,
    pos_b: Vec2,
    rad_b: f32,
    hl_b: f32,
    ang_b: f32,
) -> Option<CollisionInfo> {
    //director vecs of the capsules
    let dir_a = v2!(ang_a.cos(), ang_a.sin());
    let dir_b = v2!(ang_b.cos(), ang_b.sin());

    //each edge-point of the capsule line
    let p1 = pos_a - dir_a * hl_a;
    let q1 = pos_a + dir_a * hl_a;
    let p2 = pos_b - dir_b * hl_b;
    let q2 = pos_b + dir_b * hl_b;

    let d1 = q1 - p1; //capsule 1 vector
    let d2 = q2 - p2; //capsule 2 vector
    let r = p1 - p2; //p2p1 vec

    let a = d1.len_sq();
    let e = d2.len_sq();
    let f = d2.dot(r);
    let c = d1.dot(r);
    let b = d1.dot(d2);
    //nearest_point_1 = start + dir_a*s
    //s = (b*f -c*e)/(a*e-b²) -> vectorial equation to find the parametrical that shows as the nearest point
    let mut s;
    let mut t; //t is analog to s but in the second capsule

    let denom = a * e - b * b;
    if denom != 0.0 {
        //if the dot product is zero the lines are paralel
        s = ((b * f - c * e) / denom).clamp(0.0, 1.0); //clamping because we are dealing with segments and not infinite lines
    } else {
        //the two line are paralel so we just pick the first point of the capsule
        s = 0.0;
    }

    t = (b * s + f) / e;

    //t edge case( when the nearest point is not in the capsule segment)
    // we have to also recalculate s since we have a new t
    if t < 0.0 {
        t = 0.0;
        s = (-c / a).clamp(0.0, 1.0);
    } else if t > 1.0 {
        t = 1.0;
        s = ((b - c) / a).clamp(0.0, 1.0);
    }

    //coordinates of the virtual cicle
    let circ_a = p1 + d1 * s;
    let circ_b = p2 + d2 * t;

    collision_circle_circle(circ_a, rad_a, circ_b, rad_b)
}
