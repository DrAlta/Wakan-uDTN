use std::f32::consts::PI;

use crate::{
    rf::{Db, Dbm, SPEED_OF_LIGHT},
    Number,
};

/// Calculates received power in dBm using Free Space Path Loss
pub fn calculate_received_power(
    tx_power: Dbm,
    tx_gain: Db,
    rx_gain: Db,
    distance_m: Number,
    frequency_hz: Number,
) -> Dbm {
    let path_loss = 20.0 * ((distance_m * frequency_hz * 4.0 * PI) / SPEED_OF_LIGHT).log10();

    tx_power + tx_gain + rx_gain - path_loss
}
