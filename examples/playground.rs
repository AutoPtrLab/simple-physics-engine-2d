use macroquad::math::Vec2 as MqVec2;
use macroquad::prelude::*;
use simple_physics_engine_2d::body_shapes::body::Layer;
use simple_physics_engine_2d::prelude::Vec2;
use simple_physics_engine_2d::prelude::*;

#[macroquad::main("Simulation")]
async fn main() {
    let mut bodies: Vec<Body> = vec![
        new_rot_circle(v2!(700.0, 340.0), v2!(-400.0, -400.0), 10.0, 10.0, 1.0)
            .with_layer_bit(Layer::L0)
            .with_hitbox(),
        new_rect(v2!(300.0, 100.0), v2!(0.0, 0.0), 100.0, 50.0, 10.0, 0.5),
        new_rect(v2!(300.0, 100.0), v2!(0.0, 0.0), 100.0, 50.0, 10.0, 0.5),
        new_capsule(v2!(0.0, 340.0), v2!(0.0, 200.0), 20.0, 300.0, 10.0, 45.0, 0.5),
        new_rect(v2!(300.0, 100.0), v2!(0.0, 0.0), 100.0, 50.0, 10.0, 0.5),
        new_rot_capsule(v2!(200.0, 340.0), v2!(0.0, 200.0), 20.0, 100.0, 10.0, -30.0, 0.5),
        //new_line(v2!(0.0, 500.0), v2!(800.0, 500.0)),
        // Incluso puedes meter el suelo aquí mismo
        new_static_rect(v2!(0.0, 600.0), 1000.0, 10.0, 0.5),
    ];

    for i in 0..8 {
        bodies.push(new_rot_circle(
            v2!(700.0 + i as f32 * 10.0, 340.0),
            v2!(-400.0, -400.0),
            10.0,
            1.0,
            3.0,
        ));
    }
    loop {
        let dt = get_frame_time().min(0.016);
        //println!("{}", bodies[].ang * 180.0 / 3.141592);
        clear_background(BLACK);
        for e in &mut bodies {
            match e.shape {
                Shape::Circle { rad } => {
                    //println!("{:?}+{:?}", e.pos.x, e.pos.y);
                    draw_circle(e.pos.x, e.pos.y, rad, VIOLET);
                }
                Shape::Rectangle { width, height } => {
                    draw_rectangle(e.pos.x - width * 0.5, e.pos.y - height * 0.5, width, height, RED);
                }

                Shape::Line { p } => {
                    draw_line(e.pos.x, e.pos.y, p.x, p.y, 2.0, PINK);
                }
                Shape::Capsule { rad, half_len } => {
                    let p_x = e.ang.cos() * half_len;
                    let p_y = e.ang.sin() * half_len;
                    let p = v2!(p_x, p_y); //we calculate the vector from the center of the capsule to one of the line edges
                    let p1 = e.pos + p;
                    draw_circle(p1.x, p1.y, rad, YELLOW);
                    let p2 = e.pos - p;
                    draw_circle(p2.x, p2.y, rad, YELLOW);
                    draw_line(p1.x, p1.y, p2.x, p2.y, rad * 2.0, YELLOW);
                }
            }
        }
        if is_key_down(KeyCode::Right) {
            bodies[0].vel.x = 200.0;
        }
        if is_key_down(KeyCode::Left) {
            bodies[0].vel.x = -200.0;
        }
        if is_key_down(KeyCode::Down) {
            bodies[0].vel.y = 200.0;
        }
        if is_key_down(KeyCode::Up) {
            bodies[0].vel.y = -200.0;
        }

        update_movement(&mut bodies, dt, v2!(0.0, 90.0), 0.99, 0.95);
        let v = update_collisions(&mut bodies);
        for c in v {
            //  println!("{},{}", c.body_a_id, c.body_b_id);
        }
        bodies[0].vel = Vec2::ZERO;
        let texto_fps = format!("FPS: {}", get_fps());
        draw_text(&texto_fps, 20.0, 40.0, 30.0, GREEN);
        next_frame().await;
        // println!("{:?}", w.bodies[0]);
    }
}
