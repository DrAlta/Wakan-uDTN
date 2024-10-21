use macroquad::prelude::*;

mod time_on_air;
pub use time_on_air::{Bandwidth, LoRa};
pub mod gui;


#[macroquad::main("Draw Polygon")]
async fn main() {
    loop {
        clear_background(BLACK);
        
        let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        gui::draw_wave(center, (get_time() * 10.0) as f32, 25.0, 6, WHITE); // Example: draws a hexagon

        next_frame().await;
    }
}
/*fn main() {
    let foo = LoRa::new(1, 6, 7, Bandwidth::KHz125, 5).unwrap();
    println!("{:?}", foo.t_total());
}*/
