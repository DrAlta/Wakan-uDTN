mod time_on_air;
use time_on_air::{Bandwidth, LoRa};

fn main() {
    let foo = LoRa::new(1, 6, 7, Bandwidth::KHz125, 5).unwrap();
    println!("{:?}", foo.t_total());
}
