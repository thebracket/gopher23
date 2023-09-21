use std::collections::HashMap;

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
    let mut users = HashMap::new();
    users.insert("admin", User::new("admin", "password"));
    users.insert("herbert", User::new("herbert", "password"));
    
    if let Some(user) = users.get("herbert") {
        println!("Found user: {:#?}", user);
    }
}
