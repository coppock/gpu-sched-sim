use std::{collections::HashMap, io::stdin, num::ParseIntError, str::FromStr, time::Duration};

mod driver;
mod gpu;
mod thread;

struct Call {
    time: Duration,
    duration: Duration,
    thread: u32,
    api: thread::Api,
}

impl FromStr for Call {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split('\t').collect();
        Ok(Call {
            time: Duration::from_nanos(fields[0].parse()?),
            duration: Duration::from_nanos(fields[1].parse()?),
            thread: fields[6].parse()?,
            api: match fields[2] {
                "cuLaunchKernel" | "cudaLaunchKernel" => thread::Api::KernelLaunch {
                    duration: Duration::from_nanos(fields[10].parse()?),
                    dim: [12, 13, 14]
                        .iter()
                        .map(|x| fields[*x].parse::<u32>())
                        .product::<Result<u32, ParseIntError>>()?,
                    stream: fields[28].parse()?,
                },
                "cudaStreamSynchronize" | "cudaDeviceSynchronize" => thread::Api::Sync,
                s => {
                    eprintln!("Event `{}` not handled", s);
                    thread::Api::Other
                }
            },
        })
    }
}

fn main() {
    let mut threads: HashMap<u32, thread::Thread> = HashMap::new();
    for line in stdin().lines() {
        let Call {
            time,
            duration,
            thread,
            api,
        } = line.unwrap().parse().unwrap();
        threads.entry(thread).or_default().call(time, duration, api);
    }
}
