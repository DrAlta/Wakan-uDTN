use crate::{impl_ops_via_intermediary, Number};

/// Newtype for signal strength in dBm
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dbm(pub Number);

impl Into<Number> for Dbm {
    fn into(self) -> Number {
        self.0
    }
}
impl Into<Number> for &Dbm {
    fn into(self) -> Number {
        self.0
    }
}
impl From<Number> for Dbm {
    fn from(value: Number) -> Self {
        Self(value)
    }
}

impl_ops_via_intermediary!(Dbm, Number);
