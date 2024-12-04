use std::{cmp::Ordering, time::Duration};

use crate::driver::{Driver, Kernel, Stream};

#[derive(Default)]
pub struct Thread {
    driver: Driver,
    time: Duration,
}

impl Thread {
    pub fn call(self: &mut Self, time: Duration, duration: Duration, api: Api) {
        if let Api::KernelLaunch {
            stream,
            kernel,
        } = api
        {
            self.driver.launch(kernel, stream);
        }
        self.time += match api {
            Api::Sync => self.driver.sync(),
            _ => duration,
        };
    }
}

pub enum Api {
    KernelLaunch { stream: Stream, kernel: Kernel },
    Sync,
    Other,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Event {
    time: Duration,
    r#type: Type,
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    KernelLaunch(Stream, Kernel),
    Sync,
    KernelCompletion(Stream),
}
