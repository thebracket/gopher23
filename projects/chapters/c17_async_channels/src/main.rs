use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    tokio::spawn(async move {
        tx.send("Hello").await.unwrap();
    });
    println!("{:?}", rx.recv().await.unwrap());
}
