use crate::Number;

/// Newtype for signal strength in dBm
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dbm(pub Number);
