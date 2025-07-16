pub trait Cluster<ClusterId> {
    fn get_cluster_id(&self) -> Option<ClusterId>;
}
