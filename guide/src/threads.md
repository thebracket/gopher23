# Threads

We mentioned `Rayon`, which is a helpful crate to make using threads easy when you just want to blast out a calculation. But what if you want to do something more complicated? Rust has great support for system threads.

## Threaded Hello World

Let's start with a simple threaded hello world:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });
    handle.join().unwrap();
}
```

The `handle` is a "join handle"---your interface to the thread. You can tell if it is still running, use `join` to wait until it finishes, and get the result of the thread. In this case, we're just waiting for it to finish.

## Scoped Threads

It's never exactly clear when a thread will terminate (you may want it to run for as long as your program)---or even if a thread will terminate before or after `main` finishes. That can make lifetimes hard to calculate, and complicate dividing workloads for threaded calculation. Rust has a solution for that: scoped threads.

If you just `spawn` threads, the compiler can't guaranty when the threads will terminate. This won't compile:

```rust
use std::thread;

fn main() {
    let s = "Hello World".to_string();
    thread::spawn(|| {
        println!("{s}");
    });
}
```

Using a *thread scope* guarantees that all threads will finish by the end of the scope. This will compile:

```rust
use std::thread;

fn main() {
    let s = "Hello World".to_string();
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("{s}");
        });
    });    
}
```

## Sending Data Through Channels

One of Go's super-powers is the ability to send data between tasks using channels. Rust has channels for threads built-in, and they are easy to use. The "Multi-Producer, Single Consumer" channel is the default. You can have as many senders as you like, and a single recipient.

```rust
use std::thread;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        // In the thread, send a message and terminate
        tx.send("Hello World".to_string()).unwrap();
    });

    // In the main thread, receive one message.
    // You'd normally do `while let Ok(msg) = rx.recv() { ... }` to keep
    // receiving messages.
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
```

> You don't have to close the channel! Channels are closed when all senders are dropped. This is a great example of Rust's ownership system making your life easier.

Channels are *fast*. I recently helped a customer benchmark channels, and on my MacBook Air M1 sending/receiving over channels induced around 2 microseconds of latency (sending a 128-bit integer).

You can send most types of data over channels (the data has to be `Send`---meaning it doesn't contain any references). I like to send `enum` types, because they are easy to extend later. You can also send dynamically typed `Box<dyn Trait>` objects for heterogenous data---which is too complex to fit into a single half-day talk!

The same ownership rules apply as before:

* If you are sending data and don't need it anymore, you can move it into the channel.
* If you just need to send a copy (aka pass-by-value), you can clone it and send the clone.
* If you need need to share ownership, you can wrap it in an `Arc`.

## Protection from Data Races

Let's shoot ourselves in the foot.

### A Data Race

The [Go Playground Link](https://go.dev/play/p/2s03VKnCXm4) nearly works. Unfortunately, if you run it over and over---you'll get a variety of results. It *usually* works. If you enable Go's data-race detection, it will flag it - but the program runs on the playground without a visible warning.

Here's the program:

```go
package main

import (
	"fmt"
	"sync"
)

func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}

func chunkSlice(slice []int, chunkSize int) [][]int {
	var chunks [][]int
	for {
		if len(slice) == 0 {
			break
		}

		if len(slice) < chunkSize {
			chunkSize = len(slice)
		}

		chunks = append(chunks, slice[0:chunkSize])
		slice = slice[chunkSize:]
	}

	return chunks
}

func sumSlice(slice []int, sum *int, wg *sync.WaitGroup) {
	defer wg.Done()
	for i := 0; i < len(slice); i++ {
		*sum += i
	}
}

func main() {
	var wg sync.WaitGroup

	const numThreads = 32
	numbers := makeRange(0, 9999)
	chunks := chunkSlice(numbers, len(numbers)/numThreads)
	sum := 0
	for i := 0; i < len(chunks); i++ {
		wg.Add(1)
		go sumSlice(chunks[i], &sum, &wg)
	}
	wg.Wait()

	fmt.Println(sum)
}
```

As you can tell, I'm not a regular Go programmer---there are almost certainly better ways to do this. But it's a good example of a common pattern: divide a workload into chunks, and run each chunk in a separate thread. In this case, we are calculating the sum of a slice of numbers---and dividing up the workload.

Now let's write a Rust equivalent of the same thing:

```rust
fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u64>>(); // Equivalent to the makeRange function
    let chunks = numbers.chunks(NUM_THREADS); // Equivalent to the chunking function
    static mut SUM: u64 = 0;
    std::thread::scope(|scope| {
        for chunk in chunks {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                    SUM += *n;
                });
            });
        }
    });
}
```

This won't compile at all. Rust tells you that you can't have mutable static variables. That's a good thing: data races simply won't compile. You *have* to provide a safe interface to shared data.

## Safety with Atomics

Go has atomics for this purpose - and so does Rust. You can replace the shared variable with an atomic and all is well with the universe.

```rust
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
```

> We're going to talk about mutexes soon!