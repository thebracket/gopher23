use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::io;

#[derive(Serialize, Deserialize, Clone, Default)]
struct BlogPost {
    id: i32,
    date: DateTime<Utc>,
    title: String,
    body: String,
    author: String,
}

fn read_trim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut new_post = BlogPost::default();
    println!("Enter the title: ");
    new_post.title = read_trim();
    println!("Enter the body: ");
    new_post.body = read_trim();
    println!("Enter the author: ");
    new_post.author = read_trim();

    // Post it with Reqwest
    let client = reqwest::Client::new();
    let new_id = client
        .post("http://localhost:3001/blog/new")
        .json(&new_post)
        .send()
        .await?
        .json::<i32>()
        .await?;

    println!("New blog id: {new_id}");

    Ok(())
}
