# Enumerations

Rust has a very powerful `enum` system. Rust's enumeration system combines the simple enums found in most languages with tagged unions from C, and algebraic data types from Haskell and other functional programming languages. This combination allows for a very powerful type system that can be used to encode a wide variety of data types. It can also make code more readable.

Let's take the world's simplest login system:

```rust
fn is_login_valid(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

fn main() {
    println!("Login for admin/password is {}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {}", is_login_valid("admin", "12345"));
}
```

This works, but it's not very flexible---and it's not very readable. "true" and "false" don't really tell you much at a glance!

Let's rewrite this using an enum:

```rust
#[derive(Debug)]
enum LoginResult {
    Success,
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    if username == "admin" && password == "password" {
        LoginResult::Success
    } else {
        LoginResult::Failure
    }
}

fn main() {
    println!("Login for admin/password is {:?}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {:?}", is_login_valid("admin", "12345"));
}
```

> Notice that we've derived `Debug` for `LoginResult`. You can attach traits and functions to enums, too! Also notice that we are using `&str` for strings. This is an immutable reference to a string of characters in memory. You can also use `String` for mutable strings.

Running this shows that the result is much more readable---but you haven't really gained anything beyond having to type `LoginResult::Success` instead of `true`. Let's differentiate between admins and regular users:

```rust
#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    match (username, password) {
        ("admin", "password") => LoginResult::Success { is_admin: true },
        ("herbert", "password") => LoginResult::Success { is_admin: false },
        (_, _) => LoginResult::Failure,
    }
}

fn main() {
    println!("Login for admin/password is {:?}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {:?}", is_login_valid("admin", "12345"));
    println!("Login for herbert/password is {:?}", is_login_valid("herbert", "password"));
    println!("Login for herbert/12345 is {:?}", is_login_valid("herbert", "12345"));
}
```

> Notice that we've replaced the `if` statement with a `match`. Matching is lot like `switch` in other languages, but is a full pattern matching system. We'll dive a bit more into the power of `match` as we go on.

Enumerations can keep adding data (including other structs and enums) to each variant. The size in memory will equal the largest type in the enum. An enumeration can only be equal to one of its variants at a time---and nothing else.

Let's look at how we can use an enum on the recipient side.

```rust,editable
#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    match (username, password) {
        ("admin", "password") => LoginResult::Success { is_admin: true },
        ("herbert", "password") => LoginResult::Success { is_admin: false },
        (_, _) => LoginResult::Failure,
    }
}

fn main() {
    match is_login_valid("herbert", "password") {
        LoginResult::Success { is_admin } => {
            if is_admin {
                println!("Hello, admin!");
            } else {
                println!("Hello, user!");
            }
        }
        LoginResult::Failure => {
            println!("Login failed.");
        }
    }
}
```

You have to use `match`---or its single line variant `if let`---to extract the data from an enum. This is a good thing! It means that you can't accidentally forget to check for a failure case. It also means that you can't accidentally use the wrong data.

We can clean this up further by using a `match` statement that checks `is_admin` directly:

```rust,editable
#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    match (username, password) {
        ("admin", "password") => LoginResult::Success { is_admin: true },
        ("herbert", "password") => LoginResult::Success { is_admin: false },
        (_, _) => LoginResult::Failure,
    }
}

fn main() {
    match is_login_valid("herbert", "password") {
        LoginResult::Success { is_admin: true } => {
            println!("Hello, admin!");
        }
        LoginResult::Success { is_admin: false } => {
            println!("Hello, user!");
        }
        LoginResult::Failure => {
            println!("Login failed.");
        }
    }
}
```

Lastly, matching on enum types is *exhaustive*. Failing to match against a pattern is a compile-time error. We've added `LockedOut` as another user option---but forgotten to check it. This will fail to compile:

```rust,editable
#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
    LockedOut,
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    match (username, password) {
        ("admin", "password") => LoginResult::Success { is_admin: true },
        ("herbert", "password") => LoginResult::Success { is_admin: false },
        (_, _) => LoginResult::Failure,
    }
}

fn main() {
    match is_login_valid("herbert", "password") {
        LoginResult::Success { is_admin: true } => {
            println!("Hello, admin!");
        }
        LoginResult::Success { is_admin: false } => {
            println!("Hello, user!");
        }
        LoginResult::Failure => {
            println!("Login failed.");
        }
    }
}
```

## Two Enums You Can't Avoid!

Rust uses enums everywhere. There are two in particular that you can't avoid: `Option` and `Result`.

### Options

An `Option` is Rust's replacement for `null` values. Rust doesn't have `null` (it actually does, but only in unsafe code designed to talk to other languages). Instead, it has `Option` types. An `Option` is either `Some` value or `None`. You can't accidentally use a value that is `None` without first checking to see if it is `Some`.

```rust
fn main() {
    let some_number: Option<u8> = Some(7);
    let no_number: Option<u8> = None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);
}
```

You can't get to the contents of an `Option` without accessing the contents. There's a few ways to do this:

You can **unwrap** the value. This will panic---crash your thread---if there is nothing there. (Most programs will terminate when you `panic`---you are saying "this can't happen!" and choosing crashing over corrupting data. If you are using multiple threads, the thread will stop---and emit a stack trace on the console---but other threads can keep running.) This is a bad idea unless you are 100% sure that there is a contained value, or not having a value actually is fatal to your program:

```rust,should_panic
fn main() {
    let some_number: Option<u8> = Some(7);
    let no_number: Option<u8> = None;

    println!("Some number: {:?}", some_number.unwrap());
    println!("No number: {:?}", no_number.unwrap());
}
```

> You can replace the error message by replacing `unwrap` with `expect("I meant to do this")`.

That's really no better than accidentally accessing a `null` value in JavaScript. We can do better than that. Let's use `if let`---which is `match` with a single arm:

```rust
fn main() {
    let some_number: Option<u8> = Some(7);
    let no_number: Option<u8> = None;

    if let Some(number) = some_number {
        println!("Some number: {}", number);
    }

    if let Some(number) = no_number {
        println!("Some number: {}", number);
    } else {
        println!("There is no number!");
    }
}
```

So now you can use null-like values, and failing to check for them is a compile-time error. But what if you want to return an error? That's where `Result` comes in.

> Note that options are generic---you can put whatever you like inside!

### Results

Rust handles passing errors with the `Result` enumeration. A `Result` is either `Ok` or `Err`. You can't accidentally use a value that is `Err` without first checking to see if it is `Ok`.

```rust
fn main() {
    let some_number: Result<u8, String> = Ok(7);
    let no_number: Result<u8, String> = Err("There is no number!".to_string());

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);
}
```

Just like an `Option`, you can `unwrap` (or `expect`) a `Result` and crash/panic your thread if an error occurred. It's still a bad idea in production!

```rust,should_panic
fn main() {
    let some_number: Result<u8, String> = Ok(7);
    let no_number: Result<u8, String> = Err("There is no number!".to_string());

    println!("Some number: {:?}", some_number.unwrap());
    println!("No number: {:?}", no_number.unwrap());
}
```

You can also use `if let` to check for `Ok` and `Err`:

```rust
fn main() {
    let some_number: Result<u8, String> = Ok(7);
    let no_number: Result<u8, String> = Err("There is no number!".to_string());

    if let Ok(number) = some_number {
        println!("Some number: {}", number);
    }

    if let Ok(number) = no_number {
        println!("Some number: {}", number);
    } else {
        println!("There is no number!");
    }
}
```

We're going to use errors and `Result` types a lot as we move on. Those are the basics.