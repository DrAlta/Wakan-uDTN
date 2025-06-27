use crate::wakan::{NodeId, Parent, PlumTreeNode};

impl Parent for PlumTreeNode {
    fn get_parent(&self) -> Option<NodeId> {
        self.parent.clone()
    }
}
