use crate::{rf::Dbm, Number};

/// Calculates received power in dBm using Free Space Path Loss
pub fn calculate_received_power(
    tx_power: Dbm,
    tx_gain: Number,
    rx_gain: Number,
    distance_m: Number,
    frequency_hz: Number,
) -> Dbm {
    let fspl_db = 20.0 * distance_m.log10() + 20.0 * frequency_hz.log10() - 147.55; // FSPL constant in dB

    Dbm(tx_power.0 + tx_gain + rx_gain - fspl_db)
}
