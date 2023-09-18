# Add a Database

We're going to use `SQLite` today, rather than a big database---so you don't have to install PostgreSQL or similar on your computers. We're going to use a crate called `SQLX` that makes a lot of database work simple. We're also going to keep using our `BlogPost` type and most of the framework we've created.

Start by installing the sqlx CLI tool on your computer:

```bash
cargo install sqlx-cli
```

Next, we'll create a file to represent environment variables named `dotenv`. Most of the time, you'll run a service in a setup that gives you an environment variable for your database string. `dotenv` is a helper to make it easy to not actually have to set the variables yourself during development.

Create a file named `.env` containing:

```
DATABASE_URL=sqlite:blog.db
```

The sqlx tool makes it easy to create an empty database:

```bash
sqlx database create
```

## Migrations

It's common to handle database schema through a series of migrations. SQLX supports that, so let's make use of it. Make sure you are in your blog server directory, and run:

```bash
sqlx migrate add initial
```

This will create a `migrations` directory, with a file named `(datestamp)_initial.sql`. Let's put some SQLite SQL into there to create our table:

```sql
CREATE TABLE blog_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    title TEXT,
    body TEXT,
    author TEXT
);
```

Let's run the migration:

```bash
sqlx migrate run
```

`blog.db` now contains your data table. Let's create a second migration with some initial book data:

```bash
sqlx migrate add initial_data
```

And we'll edit the newly created `(datestamp)_initial_data.sql` file to contain:

```sql
INSERT INTO blog_posts (date, title, body, author) VALUES
    ('2021-01-01', 'A Tale of Two Cities', 'It was the best of times, it was the worst of times.', 'Dickens'),
    ('2021-01-02', 'Moby Dick', 'Call me Ishmael.', 'Melville');
```

But let's *not* run the migration yet. Instead, we're going to write some code.

## SQLX In Your Web Service

The first thing to do is add some more dependencies. SQLX in particular is picky about which features you select, so let's copy/paste:

```toml
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "sqlite", "chrono"]}
dotenv = "0.15"
```

Now open `main.rs`. At the top of the `main()` function, we're going to first read our `.env` file for environment variables - or use an environment variable if its present.

```rust
// Set the database URL
dotenv::dotenv().ok(); // Load the .env file
let database_url = std::env::var("DATABASE_URL").expect("You've not set the DATABASE_URL");
```

Calling `dotenv()` loads the `.env` file and applies any environment variables it finds. The second line queries your environment variables for `DATABASE_URL` and sets it - crashing if no database URL is found.

### Obtain a Connection Pool

We want a way to talk to the database, and a connection pool is the standard way to do it. It's always better to use a pool than to make lots of short-lived connections. We're going to use the `sqlx::sqlite::SqlitePool` type, which is a pool of SQLite connections.

```rust
// Obtain a connection pool
let connection_pool = sqlx::SqlitePool::connect(&database_url)
    .await
    .expect("Unable to connect to the database");
```

That creates a connection pool, ready for use to talk to the database.

### Automate Migrations

SQLX keeps track of which migrations have run by adding a migrations table to your database. You can automatically perform migrations with the `migrate!` macro, as follows:

```rust
// Run migrations if they haven't been applied
sqlx::migrate!()
    .run(&connection_pool)
    .await
    .expect("Unable to run database migrations");
```

Now any migrations we create will be applied---and ones that have already been applied won't be run again. No need for the CLI tool, and ideal for automated deployments.

### Database Connection Pool Dependency Injection

It's messy to have a big, static/global list of data. If you had lots of data sources, you'd have globals everywhere and a potential mess. Instead, we'll make use of Tower---one of the underlying services behind Axum---to provide dependency injection.

You can add injectable dependencies as "layers" when you build your route map. This adds the connection pool:

```rust
use axum::Extension;
let app = Router::new()
    .route("/", get(say_hello_text))
    .route("/blog/all", get(all_posts))
    .route("/blog/:id", get(get_post))
    .route("/blog/new", post(new_post))
    .layer(Extension(connection_pool));
```

Now let's adjust our `all_posts` function to use the database. We'll start by making use of a helper type from SQLX that can map `SELECT` statements to a type. We add `FromRow` to `BlogPost`:

```rust
use sqlx::FromRow;
#[derive(Serialize, Deserialize, Clone, FromRow)]
struct BlogPost {
    id: i32,
    date: String, // We changed this because SQLite doesn't have a real date type
    title: String,
    body: String,
    author: String,
}
```

Now we can rewrite our `all_posts` function:

```rust
// Return all blog posts
use axum::Json;
async fn all_posts(Extension(db) : Extension<sqlx::SqlitePool>) -> Json<Vec<BlogPost>> {
    let posts = sqlx::query_as::<_, BlogPost>("SELECT * FROM blog_posts")
        .fetch_all(&db)
        .await
        .expect("Unable to fetch posts");

    Json(posts)
}
```

Notice:

* We've added `Extension(db)` as type `Extension<sqlx::SqLitePool>` to the function signature. This matches what we inserted as a layer, so Axum will automatically `clone` the connection pool (it's `Arc`-based and designed for that) into our function.
* `sqlx::query_as` takes a type to return (leave the `_` to let it infer the input type) and returns a simple vector of blog posts.

Now run the program, and open `http://localhost:3000/blog/all` in your browser. You should see the two blog posts we added in the migration.

Close and run again, we *don't* gain two more - the migrations are stable.

So now it's relatively easy to migrate the existing items to use SQL instead of a shared data structure. Let's do that.

Obtaining a single post is as easy as using `bind` to fill out an SQL placeholder:

```rust
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
```

Adding a new post is a little more complex, but not much:

```rust
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
```

Now let's go over to our `blog_client` and make a new blog entry - and make sure it appears in our database. (Make sure the server is running first!)

```bash
cargo run
Enter the title:
Hands-on Rust
Enter the body:
I like Rust
Enter the author:
Herbert Wolverson
New blog id: 3
```

