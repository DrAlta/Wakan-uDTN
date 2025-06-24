use macroquad::prelude::*;
use wakan_sim::gui::draw_propagation_wave;

#[macroquad::main("Draw Polygon")]
pub async fn main() {
    loop {
        clear_background(BLACK);
        
        let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        draw_propagation_wave(center, (get_time() * 10.0) as f32, 25.0, 6, WHITE); // Example: draws a hexagon

        next_frame().await;
    }
}
