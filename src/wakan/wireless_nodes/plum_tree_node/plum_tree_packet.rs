use std::fmt;

use crate::wakan::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlumTreePacket {
    source: NodeId,
    count: u8,
}

impl PlumTreePacket {
    pub fn new(source: NodeId, count: u8) -> Self {
        Self { source, count }
    }
}

impl fmt::Display for PlumTreePacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{Source:{}, Count:{}}})", self.source, self.count)
    }
}
