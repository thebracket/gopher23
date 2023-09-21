fn greet_move(s: String) {
    println!("Hello, {s}");
}

fn greet_borrow(s: &String) {
    println!("Hello, {s}");
}

fn main() {
    // Cloning
    let mut name = "Hello".to_string();
    greet_move(name.clone());
    name = name + " World"; // Still Compiles
    println!("{name}");

    // Borrowing
    let mut name = "Hello".to_string();
    greet_borrow(&name);
    name = name + " World"; // Still compiles
    println!("{name}");
}
