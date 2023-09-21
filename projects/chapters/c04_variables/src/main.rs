fn main() {
    // Assign a variable
    let n = 5;
    println!("{n}");

    // Mutable
    let mut n = 5;
    n += 1;
    println!("{n}");

    // Shadowing
    let n = 5;
    let n = n + 1;
    // Where did n go?
    println!("{n}");
}
