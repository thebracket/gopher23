struct Point { x: i32, y: i32 }

fn new_point() -> Point {
    Point { x: 10, y: 20 }
}

fn new_point_on_heap<'a>() -> Box<Point> {
    Box::new(Point { x: 10, y: 20 })
}

fn main() {
    let p = new_point();
    println!("{},{}", p.x, p.y);

    let p = new_point_on_heap();
    println!("{},{}", p.x, p.y);
}