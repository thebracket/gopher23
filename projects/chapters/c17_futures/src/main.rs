async fn hello() {
    println!("Hello")
}

async fn double(n: i32) -> i32 {
    n * 2
}

async fn half(n: i32) -> i32 {
    n / 2
}

#[tokio::main]
async fn main() {
    let f = hello(); // To show the type
    hello(); // See the warning

    // Await the future call
    hello().await;

    // Wait for multiple with join!
    let (a, b) = tokio::join!(double(2), half(4));
    println!("{} {}", a, b);


}
