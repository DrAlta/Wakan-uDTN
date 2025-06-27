use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Radio(pub u8);

impl fmt::Display for Radio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Radio({})", self.0)
    }
}

impl From<u8> for Radio{
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<i8> for Radio{
    fn from(value: i8) -> Self {
        Self(value as u8)
    }
}
impl From<i32> for Radio{
    fn from(value: i32) -> Self {
        Self(value as u8)
    }
}