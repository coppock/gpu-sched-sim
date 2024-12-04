use std::{collections::HashSet, time::Duration};

#[derive(Default)]
pub struct Driver {
    launches: HashSet<u32>,
}

impl Driver {
    pub fn launch_kernel(self: &Self, duration: Duration, dim: u32, stream: u32) {}
    pub fn sync(self: &Self) -> Duration {
        Duration::default()
    }
}
