use macroquad::{color::Color, math::Vec2, shapes::draw_triangle};

// Draws a regular polygon centered at `pos`, with `sides` number of corners,
// a given `radius`, and filled with the provided `color`.
pub fn draw_polygon(pos: Vec2, radius: f32, sides: usize, color:Color) {
    // Compute the angle between each vertex of the polygon.
    // 2π radians make a full circle; divide by number of sides to get the step.
    let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

    // `start` is a fixed point used as the shared vertex for each triangle fan.
    let start = point_on_circle(0, angle_step, pos, radius);
    
    // Initialize the first vertex of the polygon
    let mut a = point_on_circle(0, angle_step, pos, radius);

    // Loop through each pair of adjacent vertices to draw triangles from `start`
    for i in 1..sides {
        let b = point_on_circle(i, angle_step, pos, radius);
        draw_triangle(
            start, // shared anchor point
            a,     // previous vertex
            b,     // current vertex
            color  // fill color
        );
        a = b;// move to next edge pair
    }
}

// A helper to calculate the (x, y) position of the i-th vertex
// on a circle centered at `pos`, with a given `radius`.
fn point_on_circle(
    i: usize, 
    angle_step: f32, 
    pos: Vec2,
    radius: f32,
) -> Vec2{
    let angle = i as f32 * angle_step; // calculate the rotation angle for this point
    let x = radius * angle.cos();      // X = cos(θ) * radius
    let y = radius * angle.sin();      // Y = sin(θ) * radius
    Vec2::new(x, y) + pos              // translate point from origin to desired position
}

// Draws a ring-shaped polygon between an inner and outer radius.
// Useful for outlining shapes or creating wave-like visuals.
pub fn draw_hollow_polygon(pos: Vec2, inner_radius: f32, thinkness: f32, sides: usize, color:Color) {
    let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

    // Start with the first pair of inner/outer points
    let mut inner_a = point_on_circle(0, angle_step, pos, inner_radius);
    let mut outer_a = point_on_circle(0, angle_step, pos, inner_radius + thinkness);

    // Loop through each polygon edge and draw two triangles to create a quad-like segment
    for i in 1..sides{

        let inner_b = point_on_circle(i, angle_step, pos, inner_radius);
        let outer_b = point_on_circle(i, angle_step, pos, inner_radius + thinkness);

        // First triangle fills half of the trapezoid
        draw_triangle(
            inner_a,
            outer_a,
            outer_b,
            color
        );
        // Second triangle completes the quad
        draw_triangle(
            outer_b,
            inner_b,
            inner_a,
            color
        );

        // Update points for next segment
        inner_a  = inner_b;
        outer_a = outer_b;
    }

    // Final patch to close the ring with one last pair of triangles
    let inner_b = point_on_circle(0, angle_step, pos, inner_radius);
    let outer_b = point_on_circle(0, angle_step, pos, inner_radius + thinkness);

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

// Decides between drawing a solid or hollow polygon, depending on geometry.
// Represents a "wave" moving outward from a central point.
pub fn draw_propagation_wave(
    pos: Vec2, 
    outer_radius: f32, 
    thickness: f32, 
    sides: usize, 
    color:Color,
) {
    if outer_radius <= thickness {
        // Not enough space to make a ring — just draw a filled polygon instead.
        draw_polygon(pos, outer_radius, sides, color);
    } else {
        let inner_radius = outer_radius - thickness;
        draw_hollow_polygon(pos, inner_radius, thickness, sides, color);
    }
}