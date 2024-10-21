use macroquad::{color::Color, math::Vec2, shapes::draw_triangle};

pub fn draw_polygon(pos: Vec2, radius: f32, sides: usize, color:Color) {
        let angle_step = 2.0 * std::f32::consts::PI / sides as f32;
    

        let mut a = foo(0, angle_step, pos, radius);
        for i in 1..sides {
            let b = foo(i, angle_step, pos, radius);
            draw_triangle(
                pos,
                a,
                b,
                color
            );
            a = b;

        }
        draw_triangle(
            pos,
            a,
            foo(0, angle_step, pos, radius),
            color
        );
}

fn foo(i: usize, angle_step: f32, pos: Vec2 ,radius: f32) -> Vec2{
    let angle = i as f32 * angle_step;
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    Vec2::new(x, y) + pos
}