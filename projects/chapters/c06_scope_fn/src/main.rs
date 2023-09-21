fn double(n: i32) -> i32 {
    n * 2 // Implicit return - no semicolon. You can also use `return n * 2;`
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        return n * 2;
    }
    0
}

fn main() {
    let n = double(5);
    println!("{n}");

    println!("{}", double_or_nothing(5));
    println!("{}", double_or_nothing(-5));
    println!("{}", {
        let n = 12;
        n
    });
}
