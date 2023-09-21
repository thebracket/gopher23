use rayon::prelude::*;

// This will run in a separate thread
fn do_some_work(id: u32) -> u32 {
    let numbers: Vec<u32> = (0 .. id * 10).collect();
    numbers.par_iter().sum()
}

async fn busy_function(id: u32) {
    let result = tokio::task::spawn_blocking(move || do_some_work(id)).await.unwrap();
    println!("{id} is busy: {result}");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut tasks = Vec::new();
    for id in 0 .. 5 {
        tasks.push(tokio::spawn(busy_function(id)));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
