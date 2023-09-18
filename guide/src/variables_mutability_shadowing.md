# Variables, Mutability and Shadowing

> We'll include the workspace setup here, but not in future projects. We'll be putting projects into their own workspace members from now on.

## Workspace Setup

Let's create a play-pen in our workspace:

```bash
cargo new variables
```

In `Cargo.toml` (the top-most one), add `variables` to the list of member projects:

```toml
[workspace]
members = [
    "hello_again",
    "variables",
]
```

Now open `variables/src/main.rs`.

## Variable Assignment

Let's add a variable to our program.

```rust
fn main() {
    let n = 5;
    println!("{n}");
}
```

Notice that we didn't give "5" a type---Rust can infer a lot of types. In the absence of anything using it, it defaults to a 32-bit signed integer---which Rust calls an `i32`.

Let's try to change it:

```rust
fn main() {
    let n = 5;
    n += 1;
    println!("{n}");
}
```

This doesn't compile, with the message "cannot assign twice to immutable variable `n`". 

### Option 1: Make the Variable Mutable

Rust variables are immutable by default. Let's make it mutable:

```rust
fn main() {
    let mut n = 5;
    n += 1;
    println!("{n}");
}
```

### Option 2: Replace the Variable with Shadowing

```rust
fn main() {
    let n = 5;
    let n = n + 1;
    // Where did n go?
    println!("{n}");
}
```

`n` was shadowed by the new `n`. The first `n` is no longer accessible (and is often optimized away completely if you don't use it). This is a common pattern in Rust code, but you have to be careful. Don't accidentally create a new variable that replaces one you still need!