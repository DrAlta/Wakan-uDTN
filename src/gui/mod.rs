use macroquad::{color::Color, math::Vec2, shapes::draw_triangle};

pub fn draw_polygon(pos: Vec2, radius: f32, sides: usize, color:Color) {
    let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

    let start = foo(0, angle_step, pos, radius);
    
    let mut a = foo(0, angle_step, pos, radius);
    for i in 1..sides {
        let b = foo(i, angle_step, pos, radius);
        draw_triangle(
            start,
            a,
            b,
            color
        );
        a = b;
    }
}

fn foo(i: usize, angle_step: f32, pos: Vec2 ,radius: f32) -> Vec2{
    let angle = i as f32 * angle_step;
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    Vec2::new(x, y) + pos
}

pub fn draw_hollow_polygon(pos: Vec2, inner_radius: f32, thinkness: f32, sides: usize, color:Color) {
    let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

    let mut inner_a = foo(0, angle_step, pos, inner_radius);
    let mut outer_a = foo(0, angle_step, pos, inner_radius + thinkness);

    for i in 1..sides{

        let inner_b = foo(i, angle_step, pos, inner_radius);
        let outer_b = foo(i, angle_step, pos, inner_radius + thinkness);

        draw_triangle(
            inner_a,
            outer_a,
            outer_b,
            color
        );
        draw_triangle(
            outer_b,
            inner_b,
            inner_a,
            color
        );
        inner_a  = inner_b;
        outer_a = outer_b;
    }
    let inner_b = foo(0, angle_step, pos, inner_radius);
    let outer_b = foo(0, angle_step, pos, inner_radius + thinkness);

    draw_triangle(
        inner_a,
        outer_a,
        outer_b,
        color
    );
    draw_triangle(
        outer_b,
        inner_b,
        inner_a,
        color
    );

}

pub fn draw_wave(pos: Vec2, outer_radius: f32, thinkness: f32, sides: usize, color:Color) {
    if outer_radius <= thinkness {
        draw_polygon(pos, outer_radius, sides, color);
    } else {
        let inner_radius = outer_radius - thinkness;
        draw_hollow_polygon(pos, inner_radius, thinkness, sides, color);
    }
}