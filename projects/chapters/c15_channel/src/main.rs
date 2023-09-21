use std::thread;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || {
        // In the thread, send a message and terminate
        tx.send("Hello World".to_string()).unwrap();
    });

    // In the main thread, receive one message.
    // You'd normally do `while let Ok(msg) = rx.recv() { ... }` to keep
    // receiving messages.
    let msg = rx.recv().unwrap();
    println!("{}", msg);
}
