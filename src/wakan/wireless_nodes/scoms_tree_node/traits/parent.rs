use crate::wakan::{NodeId, Parent, ScomsTreeNode};

impl Parent for ScomsTreeNode {
    fn get_parent(&self) -> Option<NodeId> {
        self.parent.clone()
    }
}
