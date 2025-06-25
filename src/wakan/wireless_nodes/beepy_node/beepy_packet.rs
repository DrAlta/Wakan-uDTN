use std::fmt;

use crate::wakan::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BeepyPacket(NodeId, u8);

impl BeepyPacket{
    pub fn new(source:NodeId, count:u8) -> Self {
        Self(source, count)
    }
}

impl fmt::Display for BeepyPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{Source:{}, Count:{}}})", self.0, self.1)
    }
}
