use super::super::ScomsTreeNode;
use crate::wakan::{NodeId, Parent};

impl Parent for ScomsTreeNode {
    fn get_parent(&self) -> Option<NodeId> {
        self.parent_maybe.clone()
    }
}
