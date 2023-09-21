use std::ops::Add;

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

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn print_me<S: ToString>(s: S) {
    println!("{}", s.to_string());
}

fn main() {
    // Direct initialization
    let p = Point { x: 5, y: 6 };
    println!("{}", p.x);

    // Methods and associated functions
    let p = Point::new(5, 6);
    let p2 = Point::new(10, 10);
    println!("{:?}", p);
    println!("{}", p.distance(&p2));

    // Trait Implementation
    let p = Point { x: 5, y: 6 };
    let p2 = Point { x: 10, y: 10 };
    println!("{:?}", p + p2);

    // Simple Generics
    print_me("Hello, world!");
    print_me("Hello, world!".to_string());
    print_me(5);
}