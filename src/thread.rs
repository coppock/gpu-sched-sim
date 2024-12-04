use crate::driver::Driver;
use std::time::Duration;

pub enum Api {
    KernelLaunch {
        duration: Duration,
        dim: u32,
        stream: u32,
    },
    Sync,
    Other,
}

#[derive(Default)]
pub struct Thread {
    driver: Driver,
    time: Duration,
}

impl Thread {
    pub fn call(self: &mut Self, time: Duration, duration: Duration, api: Api) {
        if let Api::KernelLaunch {
            duration,
            dim,
            stream,
        } = api
        {
            self.driver.launch_kernel(duration, dim, stream);
        }
        self.time += match api {
            Api::Sync => self.driver.sync(),
            _ => duration,
        };
    }
}
