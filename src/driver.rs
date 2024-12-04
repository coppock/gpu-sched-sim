use std::{collections::HashMap, time::Duration};

use crate::gpu::Gpu;

pub type Dim = u32;
pub type Stream = u32;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kernel {
    block_duration: Duration,
    grid_dim: Dim,
}

#[derive(Default)]
pub struct Driver {
    launches: HashMap<Stream, i32>,
}

impl Driver {
    pub fn launch(self: &mut Self, kernel: Kernel, stream: Stream, gpu: &mut Gpu) {
        gpu.launch(self.time, kernel, stream);
        *self.launches.entry(stream).or_default() += 1;
    }
    pub fn sync(self: &mut Self, gpu: &mut Gpu) -> Duration {
        let (time, stream) = gpu.complete();
        *self.launches.get_mut(&stream).unwrap() -= 1;
        time
    }
}
