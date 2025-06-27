use crate::rf::{Db, Dbm};

/// Calculates SNR (Signal-to-Noise Ratio) in dB
pub fn calculate_snr(signal: Dbm, noise_floor: Dbm) -> Db {
    Db(signal.0 - noise_floor.0)
}
