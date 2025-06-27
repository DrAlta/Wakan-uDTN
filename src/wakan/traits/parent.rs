use crate::wakan::NodeId;

pub trait Parent {
    fn get_parent(&self) -> Option<NodeId>;
}
