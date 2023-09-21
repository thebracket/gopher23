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

fn is_login_valid_iter(username: &str, password: &str) -> bool {
    get_users().iter()
        .find(|user| user.username == username && user.password == password).is_some()
}

fn main() {
    println!("Is login valid? {}", is_login_valid("herbert", "password"));
    println!("Is login valid? {}", is_login_valid_iter("herbert", "password"));
}
