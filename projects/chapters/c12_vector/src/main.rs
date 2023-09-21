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
