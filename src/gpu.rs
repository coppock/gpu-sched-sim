use std::collections::{HashMap, VecDeque};

struct Gpu {
    streams: HashMap<u32, VecDeque<()>>,
}
