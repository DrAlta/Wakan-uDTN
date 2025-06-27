mod time_on_air;
pub use time_on_air::{Bandwidth, LoRa};
pub mod graphic_frontend;
pub mod gui;
pub mod rf;
pub mod wakan;

pub type Number = ordered_f32::OrderedF32;
