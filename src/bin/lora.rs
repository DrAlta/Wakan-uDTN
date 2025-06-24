use wakan_sim::{Bandwidth, LoRa};

fn main() {
    let foo = LoRa::new(1, 6, 7, Bandwidth::KHz125, 5).unwrap();
    println!("{:?}", foo.t_total());
}