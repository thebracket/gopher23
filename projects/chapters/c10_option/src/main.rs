fn main() {
    let some_number: Option<u8> = Some(7);
    let no_number: Option<u8> = None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    // If let (one arm match)
    if let Some(number) = some_number {
        println!("Some number: {}", number);
    }

    if let Some(number) = no_number {
        println!("Some number: {}", number);
    } else {
        println!("There is no number!");
    }

    // Unwrap
    println!("Some number: {:?}", some_number.unwrap());
    println!("No number: {:?}", no_number.unwrap());
}
