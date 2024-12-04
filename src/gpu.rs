use std::{
    cmp::Ordering, collections::{BinaryHeap, HashMap, VecDeque}, time::Duration
};

use crate::driver::{Kernel, Stream};

pub struct Gpu {
    streams: HashMap<Stream, VecDeque<Kernel>>,
    events: BinaryHeap<Event>,
    time: Duration,
}

impl Gpu {
    pub fn launch(self: &mut Self, time: Duration, kernel: Kernel, stream: Stream) {
        self.events.push(Event {
            time,
            r#type: Type::KernelLaunch(stream, kernel),
        });
    }
    pub fn complete(self: &mut Self) -> (Duration, Stream) {
        loop {
            for (stream, kernels) in self.streams {
                return (self.time, stream.clone());
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Type {
    KernelLaunch(Stream, Kernel),
    BlockCompletion(Stream),
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
struct Event {
    time: Duration,
    r#type: Type,
}
