# Move by Default

This is an area that tends to confuse new Rust users. The full rule is:

> Move by Default --- Except When You Copy

That's a mouthful! Some types are `Copy` types, and are copied when you use them. The basic numeric types (`i32`, `u32`, `f32`, `f64`, `bool`) are all `Copy` types.

`Copy` types have a more relaxed ownership model. For example, this works:

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let n = 5;
    let m = double(n);
    println!("{n} {m}");
}
```

You're probably saying, "well duh - that really should work!". Let's try it with a `String`:

```rust
fn greet(s: String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(name);
    name = name + " World"; // This won't compile!
}
```

This fails to compile with the error "use of moved value: `name`".

This fails because Rust is *move by default* for *every* type that isn't trivially copyable. (The reason `i32` et al. are copyable is that it's more efficient to let the compiler just use a register, and a pointer may even use more memory than the value itself.)

So what happens when you pass a `String`---or other non-primitive type---to a function? The `String` is *moved* into the function. The function now owns the `String`, and the caller no longer does. The caller can't use the `String` anymore, because it doesn't own it anymore.

This is a very important concept in Rust. It's how Rust enforces memory safety without a garbage collector. It's also a very common source of confusion for new Rust users.

### Cloning

The first option for passing variables around is to `Clone` them. This actually requires that the type support cloning, but most of the built-in types do. (You can implement `Clone` for your own types, too.)

```rust
fn greet(s: String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(name.clone());
    name = name + " World"; // This won't compile!
}
```

The downside of doing this is that you've made two strings. If you're passing a large string around, this can be expensive. (It's not as expensive as you might think, though. Rust uses a technique called "copy on write" to avoid actually copying the string until it's modified.)

### Borrowing

A more efficient way to pass variables around is to *borrow* them. This is done with the `&` operator. For example:

```rust
fn greet(s: &String) {
    println!("Hello, {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(&name);
    name = name + " World";
}
```

Adding the ampersand (`&``) has borrowed the variable. The main function retains ownership - it's still main's variable, and it's main's responsibility to clean it up. But the function can use it, it isn't altered, and you've saved a clone---your just passing a pointer.

You can borrow mutably, too:

```rust
fn greet(s: &mut String) {
    *s = format!("Hello {s}");
    println!("{s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet(&mut name);
    name += " World";
    println!("{name}");
}
```

Notice that we are using `*` to de-reference the variable - point back at the original. You only need to do this if you aren't accessing a member of the variable.

You can only have one mutable borrow to a variable at a time. This becomes important for global variables and when you start using concurrency. The infamous "borrow checker" strictly enforces this rule to prevent data races.

### Lifetimes

Lifetimes are a more advanced topic, but it's important to understand the concept. *Every* time you borrow in Rust, the compiler calculates a *lifetime*. Rust uses this to track that the reference is valid for the entire time it's borrowed. This eliminates "use after free" and other common bugs. It also makes keeping borrowed variables around for a long time a little tricky.

This is where it's important to understand ownership:
* If you *own* a variable, you have to ensure that it will remain in memory for long enough for all references to remain valid.
* Moving a variable moves its ownership---and you can't move a variable while its borrowed.

Fortunately, there are ways to loosen these restrictions!

> **Note:** Lifetimes are a complex topic. This is a very brief introduction. You can read more about them in the [Rust Book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html).

### Compare with Go

Go has "pass by value" and "pass by reference". When you pass by value, you are effectively working on a copy of a variable---you can get the same effect by calling `clone()` in Rust. When you pass by reference, you are working on the original variable---you can get the same effect by using `&` in Rust. Rust adds a distinction between mutable and immutable references, and the borrow checker to enforce the rules.

Take the following Go code:

```go
type point struct {
  x, y int
}

func newPoint() *point {
  p := point{10, 20}
  return &p
}
```

Go's compiler performs "escape analysis" and notices that the `newPoint()` function returns a pointer to a local variable. It allocates the variable on the heap instead of the stack, and the caller is responsible for cleaning it up.

A similar Rust program won't compile at all:

```rust
struct Point { x: i32, y: i32 }

fn new_point() -> Point {
    let point = Point { x: 10, y: 20 };
    &point
}

fn main() {
    let p = new_point();
    println!("{}", p.x);
}
```

This is because from Rust's point of view it violates a golden rule: don't allocate without being told to do so. Allocations are slow, memory is a finite resource, and---especially if you are working on embedded or trying for a small footprint---you want to know about them.

In this case, you probably want to *move* the point out of the constructor and not deal with references at all:

```rust
struct Point { x: i32, y: i32 }

fn new_point() -> Point {
    Point { x: 10, y: 20 }
}

fn main() {
    let p = new_point();
    println!("{}", p.x);
}
```

If you actually *need* heap allocation, you can use a `Box`:

```rust
struct Point { x: i32, y: i32 }

fn new_point() -> Box<Point> {
    Box::new(Point { x: 10, y: 20 })
}

fn main() {
    let p = new_point();
    println!("{}", p.x);
}
```

`Box` is a "smart pointer"---similar to `unique_ptr` in C++. It wraps the pointer and provides a destructor that cleans up the memory when the `Box` goes out of scope. It also implements `Deref` and `DerefMut` so that you can use it like a regular pointer.

If you need shared ownership---which might be why you were returning a pointer in the first place---you can use `Rc` or `Arc`. We'll talk about those in a second.