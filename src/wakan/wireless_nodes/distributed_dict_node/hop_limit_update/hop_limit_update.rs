use crate::wakan::Time;

pub struct HopLimitUpdate {
    // The current maximum number of hops for discovery messages.
    pub(super) hop_limit: u8,
    // Timestamp of the last received HLU packet that either increased or confirmed the hop_limit.
    pub(super) time_of_last_hlu_received: Time,
    // Timestamp of the last time this node broadcast an HLU packet.
    pub(super) time_of_last_hlu_broadcast: Time,
    // EMA of receiving HLU packet that either increased or confirmed the hop_limit
    pub(super) hlu_mean_interval: Time,
    //  EMA of the squared differences from the mean.
    pub(super) hlu_variance: u32,
}
