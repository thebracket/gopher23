fn main() {
    // Scope Shadowing
    let n = 5;
    {
        let n = 6;
        println!("{n}");
    }
    println!("{n}");

    // Scopes and Return
    let n = {
        let mut accumulator = 0;
        for i in 1..10 {
            accumulator += i;
        }
        accumulator // No semicolon - this is the return value
    };
    println!("{n}");

    // The Unit Type
    let n = {
        println!("Hello World!");
    };
    println!("{n:?}"); // :? is a debug formatter
}
