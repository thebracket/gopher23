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