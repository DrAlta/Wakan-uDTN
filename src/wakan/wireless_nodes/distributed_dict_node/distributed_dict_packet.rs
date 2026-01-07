use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DistributedDictPacket {
    HopLimitUpdate { requested_hop_limit: u8 },
}

impl fmt::Display for DistributedDictPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DistributedDictPacket::HopLimitUpdate {
                requested_hop_limit,
            } => {
                write!(
                    f,
                    "HopLimitUpdate{{requested_hop_limit:{requested_hop_limit}}}",
                )
            }
        }
    }
}
