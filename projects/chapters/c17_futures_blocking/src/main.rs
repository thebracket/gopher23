async fn busy_function(id: i32) {
    for _ in 0 .. 5 {
        // Blocking version
        println!("{id} is busy");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
        
        // Non-blocking version:
        //tokio::time::sleep(std::time::Duration::from_secs_f32(0.1)).await;

        // Yielding version
        //tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = vec![];
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
