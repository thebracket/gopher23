# Scope Return and the Unit Type

*Everything* in Rust has a type---even nothing at all. Rust also uses scopes everywhere, so it's a good idea to understand Rust's scoping.

## Introducing Scopes

You introduce a new scope anywhere you use `{}`. For example:

```rust
fn main() {
    let n = 5;
    {
        let n = 6;
        println!("{n}");
    }
    println!("{n}");
}
```

> Note that shadowing obeys scopes. The original `n` returns to life when the inner scope ends.

When a variable is introduced inside a scope, it ceases to exist at the scope boundary. So this won't compile:

```rust
fn main() {
    let n = 5;
    {
        let i = 6;
        println!("{i}");
    }
    println!("{i}{n}");
}
```

## Scopes and Return

The last line of any scope is a `return` statement---local to that scope. If there's no semicolon, you are returning the result of the last line. For example:

```rust
fn main() {
    let n = {
        let mut accumulator = 0;
        for i in 1..10 {
            accumulator += i;
        }
        accumulator // No semicolon - this is the return value
    };
    println!("{n}");
}
```

Even scopes that don't return a value---or have a semicolon on the last line---return the unit type `()`. For example:

```rust
fn main() {
    let n = {
        println!("Hello World!");
    };
    println!("{n:?}"); // :? is a debug formatter
}
```