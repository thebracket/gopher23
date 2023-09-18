use axum::{routing::get, Router};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use axum::Extension;

use sqlx::FromRow;
#[derive(Serialize, Deserialize, Clone, FromRow)]
struct BlogPost {
    id: i32,
    date: String,
    title: String,
    body: String,
    author: String,
}

#[tokio::main]
async fn main() {
    // Set the database URL
    dotenv::dotenv().ok(); // Load the .env file
    let database_url = std::env::var("DATABASE_URL").expect("You've not set the DATABASE_URL");

    // Obtain a connection pool
    let connection_pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Unable to connect to the database");

    // Run migrations if they haven't been applied
    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .expect("Unable to run database migrations");

    // Bind the default route to the function `say_hello_text`
    use axum::routing::post;
    use axum::Extension;
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/blog/all", get(all_posts))
        .route("/blog/:id", get(get_post))
        .route("/blog/new", post(new_post))
        .layer(Extension(connection_pool));

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
async fn all_posts(Extension(db) : Extension<sqlx::SqlitePool>) -> Json<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&db)
        .await
        .expect("Unable to fetch posts");

    Json(posts)
}

// Return a single blog post by ID number
use axum::extract::Path;
async fn get_post(Extension(db) : Extension<sqlx::SqlitePool>, Path(id) : Path<i32>) -> Json<BlogPost> {
    let post = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts WHERE id = ?")
        .bind(id)
        .fetch_one(&db)
        .await
        .expect("Unable to fetch post");
    Json(post)
}

// Add a blog entry
async fn new_post(Extension(db) : Extension<sqlx::SqlitePool>, Json(post) : Json<BlogPost>) -> Json<i32> {
    use sqlx::Row;
    const SQL: &str = "INSERT INTO blog_posts (date, title, body, author) VALUES (?, ?, ?, ?) RETURNING id";
    let new_id = sqlx::query(SQL)
        .bind(post.date)
        .bind(post.title)
        .bind(post.body)
        .bind(post.author)
        .fetch_one(&db)
        .await
        .expect("Unable to insert post")
        .get::<i32, _>("id");

    Json(new_id)
}