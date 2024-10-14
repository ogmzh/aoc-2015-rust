use md5::{Digest, Md5};
use std::num::NonZero;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
pub struct Day4 {}

impl Day4 {
    pub fn part1(key: String) -> Option<i32> {
        let mut value = 0;
        while value < i32::MAX {
            let data = format!("{}{}", key, value);
            let mut hasher = Md5::new();
            hasher.update(data);
            let result = hasher.finalize();
            let res_string = format!("{:x}", result);
            if res_string.starts_with("00000") {
                return Some(value);
            } else {
                value += 1;
            }
        }
        None
    }

    pub fn part2(key: String) -> Option<i32> {
        let start_time = Instant::now();
        let mut value = 0;
        while value < i32::MAX {
            let data = format!("{}{}", key, value);
            let mut hasher = Md5::new();
            hasher.update(data);
            let result = hasher.finalize();
            let res_string = format!("{:x}", result);
            if res_string.starts_with("000000") {
                let elapsed = start_time.elapsed();
                println!("Part 2 execution time: {:?}", elapsed);
                return Some(value);
            } else {
                value += 1;
            }
        }
        let elapsed = start_time.elapsed();
        println!("Part 2 execution time: {:?}", elapsed);
        None
    }

    pub fn part2_parallel_v2(key: String) -> Option<i32> {
        let start_time = Instant::now();
        let result = Arc::new(AtomicI32::new(0));
        let found = Arc::new(AtomicBool::new(false));
        let num_threads = thread::available_parallelism()
            .unwrap_or(NonZero::new(8).unwrap())
            .get();

        println!("Using {num_threads} threads");

        let mut handles = vec![];

        (0..num_threads).for_each(|thread_id| {
            let key = key.clone();
            let found = Arc::clone(&found);
            let result = Arc::clone(&result);

            let handle = thread::spawn(move || {
                let mut value = thread_id as i32;
                while value < i32::MAX {
                    if found.load(Ordering::Relaxed) {
                        break;
                    }
                    let data = format!("{}{}", key, value);
                    let mut hasher = Md5::new();
                    hasher.update(data);
                    let result_hash = hasher.finalize();
                    let res_string = format!("{:x}", result_hash);

                    if res_string.starts_with("000000") {
                        result.store(value, Ordering::Relaxed);
                        found.store(true, Ordering::Relaxed);
                        break;
                    } else {
                        value += num_threads as i32;
                    }
                }
            });
            handles.push(handle);
        });

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());

        if found.load(Ordering::Relaxed) {
            let elapsed = start_time.elapsed();
            println!("Part 2 execution time: {:?}", elapsed);
            return Some(result.load(Ordering::Relaxed));
        }
        None
    }
}
