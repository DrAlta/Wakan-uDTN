use super::super::LocalMinimumClusterNode;
use crate::wakan::{NodeId, Parent};
impl Parent for LocalMinimumClusterNode {
    fn get_parent(&self) -> Option<NodeId> {
        self.parent_maybe.clone()
    }
}
