# Scopes and Functions

So what about functions? Functions introduce a scope, and it acts like scopes everywhere else---including the return:

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let n = double(5);
    println!("{n}");
}
```

`n` in the `double` function is completely independent of `n` in `main`. Unlike an in-block scope, you have to specify the return type of functions that return a value. Functions that don't return a value return `()`, but you don't need to specify it.

You can use explicit return statements, too:

```rust
fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn main() {
    println!("{}", double_or_nothing(5));
    println!("{}", double_or_nothing(-5));
    println!("{}", {
        let n = 12;
        return n;
    });
}
```

> It's generally more idiomatic to use implicit returns most of the time, and reserve `return` for marking early returns.
