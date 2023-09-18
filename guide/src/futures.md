# Futures

Async rust follows the "async/await" model. Async tasks *await* the results of other async tasks.

> You can always call synchronous tasks from an async context. You can't call async tasks from anything other than an async context.

## Awaiting the Future

So let's start with a simple async task:

```rust
async fn hello() {
    println!("Hello")
}

#[tokio::main]
async fn main() {
    hello();
}
```

What's this? Nothing happened! That's because the `async` keyword actually changes the signature of a function. It no longer just runs, it returns a `Future`. A `Future` represents a task that can be cancelled, executed and awaited. So we need to *await* the result of the `hello` function:

```rust
async fn hello() {
    println!("Hello")
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

That executes the function. What's actually happening?

1. The `hello()` call returns a `Future`.
2. The `await` keyword tells the runtime to execute the `Future`, and wait for it to be done.
3. The runtime switches your task to being "busy", and adds `hello` to the active list.
4. The runtime executes `hello` until it returns a value (remember even not returning actually returns `()`).
5. Now that `hello` is done, `main` becomes ready - and will execute as soon as it is at the top of the task list.

## Waiting on Multiple Futures

Tokio's `join` macro makes it easy to run lots of tasks at once and wait for all of them to finish:

```rust
async fn double(n: i32) -> i32 {
    n * 2
}

async fn half(n: i32) -> i32 {
    n / 2
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(double(2), half(4));
    println!("{} {}", a, b);
}
```

> If you are using a multi-threaded runtime, the tasks may execute concurrently. If you are using a single-threaded runtime, the tasks will execute in order.

## The Blocking Problem

Async tasks run until they wait for something. That means that in a single-threaded environment, you can stall execution by running for too long. You can also really snarl things up by executing synchronous code that blocks for a long time, or by running CPU heavy tasks for long periods of time.

We're going to use `tokio::spawn`, which creates a new async task and executes it immediately (returning a handle you can wait for, but you don't have to).

Let's deliberately mess things up to illustrate:

```rust
async fn busy_function(id: i32) {
    for _ in 0 .. 5 {
        println!("{id} is busy");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = vec![];
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
```

The whole program turns into a serial process, and isn't really async at all. Using `thread::sleep` actually puts the entire thread---including the Tokio executor---to sleep. So the executor can't run any other tasks until the sleep is done. You might get away with that in a multi-threaded environment, but it's not a great idea!

In this case, you can substitute Tokio's own async-friendly version of sleep to achieve the same thing (Tokio provides much of the standard library in an async form):

```rust
async fn busy_function(id: i32) {
    for _ in 0 .. 5 {
        println!("{id} is busy");
        tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = vec![];
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
```

Notice how even though we're single threaded, the tasks have interleaved and all is right once more in the async world? Each `sleep` *awaits* the result of the sleep, allowing the other tasks to run.

## Yielding

If you have a long-running CPU-bound task, you can elect to periodically suspend execution and let other tasks run. This is called *yielding*.

```rust
async fn busy_function(id: i32) {
    for _ in 0 .. 5 {
        println!("{id} is busy");
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = vec![];
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
```

Yielding isn't ideal, but gives you a safety net if you need to do some CPU-heavy work.

## Spawn Blocking

If you have a function that needs to block (whether because it's going to do some CPU intensive work, it isn't async friendly, etc.), you can use `tokio::task::spawn_blocking` to run it in a separate thread and await the result:

```rust
use rayon::prelude::*;

// This will run in a separate thread
fn do_some_work(id: u32) -> u32 {
    let numbers: Vec<u32> = (0 .. id * 10).collect();
    numbers.par_iter().sum()
}

async fn busy_function(id: u32) {
    let result = tokio::task::spawn_blocking(move || do_some_work(id)).await.unwrap();
    println!("{id} is busy: {result}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = Vec::new();
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
```

So there you have it: you can bridge the async and sync worlds with `spawn_blocking`. You can have the power of system threads, and the throughput of async tasks---all at once. You can even call `rayon` and other helpers to make threading easy.

> Note: you can use `rayon` inside `async` functions, too. You don't have to spawn it into a blocking thread---but if its going to take a while, you should.