# Cleaning Up the Garbage

*Rust doesn't have a garbage collector*. When you create a string, it's created on the heap---and has to be deleted. So why didn't we have to cleanup after ourselves with our strings?

This ties into the *ownership* concept we talked about, and why it's important to understand who owns a variable. Rust uses a concept called *RAII* (Resource Acquisition Is Initialization) to automatically clean up after itself. When a variable goes out of scope, Rust calls the `drop` function on it. This is where the cleanup happens. `drop` is implemented by a trait called `Drop`. Let's quickly implement our own `drop` just to see how it works:

```rust
struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn main() {
    let mut my_struct = MyStruct { name: "Hello".to_string() };
}
```

When you run this, you learn that "Hello" was dropped. This is because `my_struct` goes out of scope at the end of `main`, and Rust calls `drop` on it. Rust will call `drop` on all members of the structure being dropped, too. So `String` receives a `drop` call, and it frees the memory it allocated.

Let's illustrate this a bit more by using a function with a moved value:

```rust
struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn greet(s: MyStruct) {
    println!("Hello, {}", s.name);
}

fn main() {
    let mut my_struct = MyStruct { name: "Hello".to_string() };
    greet(my_struct);
    println!("Exiting main function");
}
```

The output shows exactly when your structure ceased to exist:

```
Hello, Hello
Dropping Hello
Exiting main function
```

So there's no garbage collector---but it feels like you have one, because RAII cleans up variables as soon as they are unused. This is a very powerful concept, and it's one of the reasons Rust is so fast.

### Reference Counting

Sometimes, you *want* to share ownership---and have a variable expire as soon as its done. Rust has a built-in reference counting system to handle this. It's called `Rc`, and it's used like this:

```rust
use std::rc::Rc;

struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn greet(s: Rc<MyStruct>) {
    println!("Hello, {}", s.name);
}

fn main() {
    let mut my_struct = Rc::new(MyStruct { name: "Hello".to_string() });
    greet(my_struct.clone());
    greet(my_struct.clone());
    println!("Exiting main function");
}
```

Notice how you `clone` the `Rc`? That's how you make a new reference. There's still only one `MyStruct`---but every `clone` adds to the reference count. Whenever it drops, the reference count is decremented. When the reference count hits zero, your structure is dropped.

> There's also `Arc`, which uses an atomic for safety and can be sent between threads. We'll talk about atomics later.

So Rust still doesn't have a garbage collector, but you have options: you can rely on RAII to clean up after you, or you can use reference counting to share ownership. Either way, you don't have to worry about memory leaks.

> Fun fact: memory leaks are *not* included in Rust's memory guarantees. There's even a couple of commands to explicitly leak memory, if you want to! With that said, RAII makes it hard to leak memory by accident.

## Some General Rules for Ownership

When you create variables, think about ownership.

* If the variable is going to stay local, you can forget all about it as soon as you are done with it.
* If you are sending the variable somewhere and won't use it again, *move* it to the destination. It's now the destination's problem!
* If you are sending the variable somewhere, but it is still yours---send a borrowed reference.
* If you have high fan-out, or a variable is a truly shared resource, use reference counting. That way you don't have to worry about ownership---the variable will be destroyed when everyone is done with it. `Rc` could be called "shared ownership".