fn main() {
    let some_number: Result<u8, String> = Ok(7);
    let no_number: Result<u8, String> = Err("There is no number!".to_string());

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    // If let (one arm match)
    if let Ok(number) = some_number {
        println!("Some number: {}", number);
    }

    if let Ok(number) = no_number {
        println!("Some number: {}", number);
    } else {
        println!("There is no number!");
    }

    // Unwrap (and panic)
    println!("Some number: {:?}", some_number.unwrap());
    println!("No number: {:?}", no_number.unwrap());
}
