# Grouping Data with Structs

You can combine data together into a single type using a `struct`. A `struct` is a collection of named fields. You can access struct members with the dot notation:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 6 };
    println!("{}", p.x);
}
```

## Structs can Have Functions, Too

There are two major types of functions associated with a type: *methods* and *associated functions*. Methods are functions that take `self` as the first parameter. Associated functions are functions that don't take `self` as the first parameter. Associated functions are often used as constructors.

Let's add a constructor to our `Point` type:

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let p = Point::new(5, 6);
    println!("{:?}", p);
}
```

And lets add a method to our `Point` type:

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> f64 {
        let x = (self.x - other.x) as f64;
        let y = (self.y - other.y) as f64;
        (x * x + y * y).sqrt()
    }
}

fn main() {
    let p = Point::new(5, 6);
    let p2 = Point::new(10, 10);
    println!("{:?}", p);
    println!("{}", p.distance(&p2));
}
```

> Notice we are using `as` to convert to a 64-bit floating point number before we run the `sqrt` function. Rust is very strict about type conversions, and you have to be explicit about them.

## Traits

Struct's don't so a lot by themselves. They gain functionality by implementing *traits*---an *interface* in most other languages. Unlike Go, traits/interfaces are explicit. Structures don't conform to a trait unless you explicitly implement it. (There are some exceptions---notably determining if a type if safe to send between threads)

For example, the ability to debug-print a structure requires that the structure implement the `Debug` trait. You can easily implement the common traits using the `derive` attribute:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 6 };
    println!("{:?}", p);
}
```

Likewise, you can't clone a type until it implements `Clone`. You can implement `Clone` manually, but it's usually easier to derive it:

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 5, y: 6 };
    let p2 = p.clone();
    println!("{:?} {:?}", p, p2);
}
```

When you call `#[derive(Debug)]` on `Point`, you are actually writing out a full trait implementation. Lots of traits provide a derive macro to save your fingers.

## Implementing Operators

You can use Rust's trait system to implement operators. For example:

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 6 };
    let p2 = Point { x: 10, y: 10 };
    println!("{:?}", p + p2);
}
```

Unlike C++, Rust doesn't allow you to overload operators for arbitrary types. You can only implement operators for types you own. It's a *really bad idea* to overload operators and make them do crazy things. C++ is a living testament to that!

## Simple Generic Functions with Traits

Traits are the building blocks of much of Rust, and can be combined in lots of interesting ways. That's an advanced topic, but here's a simple example that everyone finds useful:

```rust
fn print_me<S: ToString>(s: S) {
    println!("{}", s.to_string());
}

fn main() {
    print_me("Hello, world!");
    print_me("Hello, world!".to_string());
    print_me(5);
}
```

The `<S: ToString>` specifies that *any* variable that implements the `ToString` trait can be passed to `print_me`. This is really handy for constructors and not making your user care about passing a `String` or a `&str`.