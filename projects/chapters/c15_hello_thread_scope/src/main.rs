use std::thread;

fn main() {
    let s = "Hello World".to_string();
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("{s}");
        });
    });    
}
