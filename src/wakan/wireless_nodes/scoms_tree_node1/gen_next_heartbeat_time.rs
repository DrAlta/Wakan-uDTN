use std::hash::{DefaultHasher, Hash, Hasher};

use crate::wakan::Time;

pub fn gen_next_heartbeat_time(time: Time) -> Time {
    let mut hasher = DefaultHasher::new();
    time.hash(&mut hasher);
    let hash = hasher.finish();
    (hash % 29) as Time + 5
}
