# Async Rust

There are two forms of concurrency built into Rust:

* **System Threads**. You used these with `Rayon`. They exactly map to threads created by the operating system, and are the most flexible form of concurrency. Threads are best for CPU-intensive operations. You can't have too many of them, each gets its own stack, and scheduling entry in the operating system.
* **Async Tasks**. This is a more Go-like construct. It's cooperative multi-tasking, breaking up one or more threads into "tasks". Async assumes that you are *waiting* for other services: a database, a network, a file, etc. Even single-threaded, async tasks can provide amazing overall throughput for something like a server that spends most of its time waiting for something else.

Rust is agnostic about "async runtime". Go, C#, et al. all make some decisions for you regarding how async tasks are scheduled. Rust leaves that up to you. There are a number of async runtimes available for Rust, and you can even write your own. `Smol` is great for tiny, embedded tasks. On the other end of the spectrum `Tokio` provides an all-singing, all-dancing async framework well-suited for enterprise development.

Tokio can look a lot like Go:

* Tokio creates a pool of worker threads, one per CPU (by default - this can be changed).
* Each worker thread has a task pool and will execute the next available task.
* Worker threads can "steal" work from each other, providing both parallel execution and concurrency with little user intervention.

## Firing up an Async Runtime

Async functions can *only* be called from inside an async environment. That means that you have to start a *runtime* and give it control of execution before you can start using async tasks.

The long, ugly version of starting up a Tokio runtime looks like this:

```rust
use tokio::runtime::Runtime;

fn main() {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Your async program here
    });
}
```

This is verbose, but you have a lot of options. For example, you can create a single-threaded runtime (and have more than one, if you wish):

```rust
use tokio::runtime;

fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        // Your async program here
    });
}
```

Or you can create a multi-threaded runtime and really tune the options (multi-threaded is the default):

```rust
fn main() {
    let rt = runtime::Builder::new_multi_thread()
        // YOU DON'T HAVE TO SPECIFY ANY OF THESE
        .worker_threads(4)  // 4 threads in the pool
        .thread_stack_size(3 * 1024 * 1024) // You can set the stack size
        .event_interval(61) // You can increase the I/O polling frequency
        .global_queue_interval(61) // You can change how often the global work thread is checked
        .max_blocking_threads(512) // You can limit the number of "blocking" tasks
        .max_io_events_per_tick(1024) // You can limit the number of I/O events per tick
        // YOU CAN REPLACE THIS WITH INDIVIDUAL ENABLES PER FEATURE
        .enable_all()
        // Build the runtime
        .build()
        .unwrap();

    rt.block_on(async {
        // Your async program here
    });
}
```

So if you *need* all this functionality, it's there for you. Most of the time, you just want to write some async code and have it run fast! Tokio has you covered:

```rust
#[tokio::main]
async fn main() {
    // Do something
}
```