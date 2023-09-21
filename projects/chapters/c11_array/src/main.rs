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
