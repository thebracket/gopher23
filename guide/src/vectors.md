# Vectors of Users

It's quite unlikely that in production you'll want to recompile every time you add a user, let alone keep a user list small enough to fit on the stack!

Rust's `Vec` type---vector---is a growable array. It's actually a type annotation, a capacity and a pointer to the data on the heap. Whenever you exceed its capacity, it doubles in size.

So let's take our previous example, and use a vector instead:

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

fn get_users() -> Vec<User> {
    vec![
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
```

Notice that the iterator didn't change, and we just used the `vec!` macro to initialize the vector. We also changed the return type of `get_users` to `Vec<User>`.

Now we can add and remove users from the vector:

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

fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password"),
        User::new("herbert", "password"),
    ]
}

// Notice we're now passing &[User]. This is a *slice*---a reference to a contiguous
// block of Users in memory.
fn is_login_valid(users: &[User], username: &str, password: &str) -> bool {
    users.iter()
        .find(|user| user.username == username && user.password == password).is_some()
}

fn main() {
    let mut users = get_users();
    users.push(User::new("new_user", "password"));
    // You can convert a vector into a slice with &my_vec. You can even
    // limit the part of the vector with &my_vec[0..2]
    println!("Is login valid? {}", is_login_valid(&users, "new_user", "password"));
}
```

> C programmers who come to Rust tend to complain that everything is a vector. Vec really is everywhere in Rust. Many of the other collection types even use Vec as a basis. Vec is *really* efficient: it's guaranteed that the entries will be contiguous in memory, making it very cache friendly. It's also guaranteed that the entries will be in the same order as you inserted them, which is not true for HashMaps and other collections.

> Go programmers have access to slices, which work a lot like Rust vectors.

## Some Helpful Vector and Iterator Functions

### Length Checks

You can use `vec.len()` to get the length of a vector, and `vec.is_empty()` to check if it's empty. It's faster to check `is_empty()` than `vec.len() == 0`.

### Pre-allocating Capacity

If you know how many items you need in a vector, you can create it with `Vec::with_capacity(n)` or call `vec.reserve(n)` to reserve `n` new slots in the vector. This avoids repeated re-allocation of memory as you add.

### Shrinking

You can also shrink a vector with `vec.shrink_to_fit()`. This will shrink the vector to its minimum size, which is useful if you've just removed a lot of items.

### Sorting

Sorting vectors is built-in:

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    println!("Sorted vec: {:?}", vec);
}
```

You can even sort complex types by providing your own comparison function:

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

fn main() {
    let mut vec = vec![
        User::new("herbert", "password"),
        User::new("admin", "password"),
        User::new("new_user", "password"),
    ];
    vec.sort_by(|a, b| a.username.cmp(&b.username));
    println!("Sorted vec: {:#?}", vec);
}
```

### Some Helpful Iterator Functions

Iterators provide a ton of useful functionality for working with data. Here are some of the most useful:

```rust
fn main() {
    // You can collect ranges into vectors to get a sequential list of numbers:
    let numbers: Vec<u32> = (0..100).collect();
    println!("{numbers:?})");

    // You can sum, min and max iterators:
    let min = numbers.iter().min().unwrap(); // Unwrap in case there isn't one
    let max = numbers.iter().max().unwrap();
    let sum: u32 = numbers.iter().sum();
    println!("{min}, {max}, {sum}");

    // You can take part of an iterator, map it into something else and collect
    // it into another collection
    let squares: Vec<u32> = numbers.iter().take(10).map(|n| n * n).collect();
    println!("{squares:?}");

    // You can divide an iterator into two parts with partition:
    let (even, odd): (Vec<u32>, Vec<u32>) = numbers.iter().partition(|n| *n % 2 == 0);

    // You can divide a vector into nearly-equal-sized chunks for parallel processing:
    let chunks: Vec<Vec<u32>> = numbers.chunks(10).map(|chunk| chunk.to_vec()).collect();
    println!("{chunks:?}");
}
```
