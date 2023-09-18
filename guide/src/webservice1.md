# Web Service

> We're going to be building a webserver. The Rust Playground doesn't support external network connections---for obvious security reasons. So the examples in this section won't run in-place. You'll need to either build them yourself as you follow along, for go into the `projects/blog_server` directory and run them from there.

`Axum` is a high-performance web-server built by the Tokio team. It's very powerful, and consistently in the top-10 performing webservers. It's also easy to use, and provides the enterprise features you need.

## Hello World

> The code for this is in the `projects/hello_server` directory.

Let's start by building a hello world web service.

Start by adding some dependencies:

```bash
cargo add tokio -F full
cargo add axum
```

Then let's build a server. `Axum` is a lot like `Express` on `NodeJS` to work with. You define some routes, start listening to the network and start the web service:

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Bind the default route to the function `say_hello_text`
    let app = Router::new().route("/", get(say_hello_text));

    // Listen on localhost, port 3001
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
```

If you go to [localhost:3000](http://localhost:3000) you should see the text `Hello, world!` in your browser. Not too impressive, but you've built a web server in 13 lines of code!

## Adding Some Blog Data

> The code for this is in the `projects/blog_server` directory.

Let's add a couple more dependencies:

```bash
cargo add serde -F derive
cargo add chrono -F serde
cargo add once_cell
```

The "derive" flag for Serde adds the ability to `#[derive(Serialize, Deserialize)]`. Without it, you have to do it the hard way - and that's no fun. Chrono is a library of time utilities; adding the `serde` flag makes the time types serializable and deserializable with no additional effort on your part.

`once_cell` is becoming part of the standard library, but isn't quite there yet. You need it to lazy-initialize a global variable.

Let's start by defining what a blog post looks like:

```rust
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
```

Now let's create a global, shared list of posts. Don't worry, we'll be turning this into a nice idiomatic, database-backed structure in a moment.

This is actually a bit of a mouthful:

```rust
use tokio::sync::Mutex;
use std::sync::Arc;
use once_cell::sync::Lazy;

static POSTS: Lazy<Arc<Mutex<Vec<BlogPost>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});
```

So what's happening here?

* On the outermost layer, we have a `Lazy`. Lazy is a special type---provided by `once_cell` but moving into the standard library that provides *lazy initialization* of a structure. It will be initialized by running the provided closure the first time it's accessed.
* The next layer in uses an `Arc`. We talked about those---they add a thread-safe reference counter, allowing you to `clone` the structure as many times as you want - and only have one copy. You don't *really* need the `Arc` yet, but it'll help when we make this more idiomatic.
* A `Mutex` is just like a Go mutex, except that Rust mutexes *wrap* a piece of data. You *can't* get to to the inner data without locking the mutex---making it basically impossible to cause a race condition. You also can't mutably access data *without* a synchronization primitive (`Mutex`, `RwLock`, etc.). The resulting data race will simply fail to compile.
* Then we have a vector of our `BlogPost` type.

We probably want to start with some initial data, so let's replace the initializer:

```rust
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
```

Now let's create a route that returns all of the blog posts. We can make use of Axum having built-in support for Serde to make this quite easy:

```rust
// Return all blog posts
use axum::Json;
async fn all_posts() -> Json<Vec<BlogPost>> {
    // Lock the mutex
    let lock = POSTS.lock().await;
    Json(lock.to_vec())
}
```

And we have to wire it up to a route:

```rust
async fn main() {
    // Bind the default route to the function `say_hello_text`
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/blog/all", get(all_posts));
```

Now run the program with `cargo run`, and goto [http://localhost:3001/blog/all](http://localhost:3001/blog/all). You will see a nicely JSON formatted list of all your blog posts.

Let's add an endpoint to retrieve a post by ID number:

```rust
// Return a single blog post by ID number
use axum::extract::Path;
async fn get_post(Path(id) : Path<i32>) -> Json<BlogPost> {
    let lock = POSTS.lock().await;
    let post = lock.iter().find(|post| post.id == id).cloned().unwrap();
    Json(post)
}
```

Notice how we use a `Path` type from Axum's extractors for the parameter? This allows you to connect it to part of the URL to extract the requested ID number. Let's wire up the route:

```rust
let app = Router::new()
    .route("/", get(say_hello_text))
    .route("/blog/all", get(all_posts))
    .route("/blog/:id", get(get_post));
```

Run the program, and you can go to [http://localhost:3001/blog/1](http://localhost:3001/blog/1) and see the first blog post. We're not handling errors gracefully yet (try to go to number 3), but the functionality is there.

## Adding Posts

How about supporting `POST` to add a post? We'll use this as an excuse to also create a REST client.

Let's start by supporting the `POST` of a new blog entry:

```rust
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
```

And we'll set it up with a route:

```rust
use axum::routing::post;
let app = Router::new()
    .route("/", get(say_hello_text))
    .route("/blog/all", get(all_posts))
    .route("/blog/:id", get(get_post))
    .route("/blog/new", post(new_post));
```

> It's the same drill for the other HTTP verbs!

Now let's create a REST client to add posts. We'll make a new project:

```bash
cargo new blog_client
# Remember to add the blog client to your workspace members!
cd blog_client
cargo add tokio -F full
cargo add reqwest -F json
cargo add serde -F derive
cargo add chrono -F serde
cargo add anyhow
```

We've added two new crates:

* `reqwest` is a web client. It's a very nice, easy to use client that supports async/await. `Hyper` is the underlying HTTP client, but it's a bit more low-level.
* `anyhow` is a crate that makes it easy to return errors from functions. It's a bit of a mouthful, but it's a lot easier than writing your own error types.

In a real project, we'd make a library and share the data types. For today, we're just going to copy/paste our `BlogPost` type into `main.rs` in the client. We'll add one more derivation: `Default`. This allows you to make an empty record easily.

```rust
#[derive(Serialize, Deserialize, Clone, Default)]
struct BlogPost {
    id: i32,
    date: DateTime<Utc>,
    title: String,
    body: String,
    author: String,
}
```

We'll also add a helpful function to retrieve text from the console, and trim out the `\n` at the end:

```rust
fn read_trim() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
```

Finally, we'll obtain the data from the user - and add a blog post:

```rust
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
```

Now make sure that the server is running, and let's add a post. You should see something like this:

```bash
Enter the title: 
Hands-on Rust
Enter the body:
Rust is a fun language
Enter the author:
Wolverson
New blog id: 3
```

If you go to [http://localhost:3001/blog/3](http://localhost:3001/blog/3) you should see your new blog post.

So now you have the basics to create a webservice. But a service that forgets everything when you restart it isn't all that useful - so let's add a database.