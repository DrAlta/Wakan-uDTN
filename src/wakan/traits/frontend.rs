pub trait Frontend {
    type Settings;
    const SETTINGS: <Self as Frontend>::Settings;
    fn tick_sim(&mut self) -> impl std::future::Future<Output = ()>;
}
