use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u64>>(); // Equivalent to the makeRange function
    let chunks = numbers.chunks(NUM_THREADS); // Equivalent to the chunking function
    static SUM: AtomicU64 = AtomicU64::new(0);
    std::thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                    SUM.fetch_add(*n, Ordering::Relaxed);
                });
            });
        }
    });
    println!("{}", SUM.load(Ordering::Relaxed));
}
