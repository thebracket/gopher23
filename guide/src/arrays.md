# Arrays of Users

We grouped data together into structures, and we grouped results together with an enum. It's pretty unlikely that your system will only have one user. Let's start with the simplest type of collection, an array.

Arrays are a *fixed size* list of items, and are stored on the stack---so you can't have a *huge* list of items. You can create an array with the `[]` syntax:

```rust
#[derive(Debug)]
struct User {
    username: String,
    password: String,    
}

impl User {
    fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

fn get_users() -> [User; 2] {
    [
        User::new("admin", "password"),
        User::new("herbert", "password"),
    ]
}

fn main() {
    let users = get_users();
    println!("{users:#?}"); // #? is "pretty print debug"
}
```

Coming from a strictly imperative language, you might be tempted to write a login test as follows:

```rust
#[derive(Debug)]
struct User {
    username: String,
    password: String,    
}

impl User {
    fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

fn get_users() -> [User; 2] {
    [
        User::new("admin", "password"),
        User::new("herbert", "password"),
    ]
}

fn is_login_valid(username: &str, password: &str) -> bool {
    let users = get_users();
    for user in users {
        if user.username == username && user.password == password {
            return true;
        }
    }
    false
}

fn main() {
    println!("Is login valid? {}", is_login_valid("herbert", "password"));
}
```

It works, and there's nothing wrong with it. A more Rustacean approach would use an *iterator*. Iterators are a Rust superpower---they provide a generic way to loop over different data-types, they're lazy (they won't do anything until they move to the next entry), and they provide a lot of convenient methods for working with data.

Let's make use of the `find` iterator function instead:

```rust
#[derive(Debug)]
struct User {
    username: String,
    password: String,    
}

impl User {
    fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

fn get_users() -> [User; 2] {
    [
        User::new("admin", "password"),
        User::new("herbert", "password"),
    ]
}

fn is_login_valid(username: &str, password: &str) -> bool {
    get_users().iter()
        .find(|user| user.username == username && user.password == password).is_some()
}

fn main() {
    println!("Is login valid? {}", is_login_valid("herbert", "password"));
}
``````

> There are actually *two* types of iterator acquisition functions. `iter()` iterates over *references* to the data. `into_iter()` moves each item into the iterator---destroying the original. We could use `into_iter()` safely here, but it's a good habit to use `iter()` unless you need to move the data.