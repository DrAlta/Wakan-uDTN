use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct NodeId(pub u32);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node({})", self.0)
    }
}

impl From<u32> for NodeId{
    fn from(value: u32) -> Self {
        Self(value)
    }
}impl From<usize> for NodeId{
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}