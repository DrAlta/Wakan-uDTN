use crate::wakan::{traits::Cluster, wireless_nodes::LocalMinimumClusterNode};

impl Cluster<u64> for LocalMinimumClusterNode {
    fn get_cluster_id(&self) -> Option<u64> {
        todo!()
    }
}
