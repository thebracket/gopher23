use axum::{routing::get, Router};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
struct BlogPost {
    id: i32,
    date: DateTime<Utc>,
    title: String,
    body: String,
    author: String,
}

use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;

static POSTS: Lazy<Arc<Mutex<Vec<BlogPost>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(vec![
        BlogPost {
            id: 1,
            date: Utc::now(),
            title: "A Tale of Two Cities".to_string(),
            body: "It was the best of times, it was the worst of times.".to_string(),
            author: "Dickens".to_string(),
        },
        BlogPost {
            id: 2,
            date: Utc::now(),
            title: "Moby Dick".to_string(),
            body: "Call me Ishmael.".to_string(),
            author: "Melville".to_string(),
        },
    ]))
});

#[tokio::main]
async fn main() {
    // Bind the default route to the function `say_hello_text`
    use axum::routing::post;
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/blog/all", get(all_posts))
        .route("/blog/:id", get(get_post))
        .route("/blog/new", post(new_post));

    // Listen on localhost, port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Return a static string
async fn say_hello_text() -> &'static str {
    "Hello, world!"
}

// Return all blog posts
use axum::Json;
async fn all_posts() -> Json<Vec<BlogPost>> {
    // Lock the mutex
    let lock = POSTS.lock().await;
    Json(lock.to_vec())
}

// Return a single blog post by ID number
use axum::extract::Path;
async fn get_post(Path(id) : Path<i32>) -> Json<BlogPost> {
    let lock = POSTS.lock().await;
    let post = lock.iter().find(|post| post.id == id).cloned().unwrap();
    Json(post)
}

// Add a blog entry
async fn new_post(Json(post) : Json<BlogPost>) -> Json<i32> {
    let mut post = post; // Move it into a mutable variable
    let mut lock = POSTS.lock().await; // Lock the mutex

    // Find the maximum ID # and add one
    let new_id = lock.iter().max_by(|a,b| a.id.cmp(&b.id)).unwrap().id + 1;
    post.id = new_id;

    // Add the post
    lock.push(post);

    // Return the new ID number
    Json(new_id)
}