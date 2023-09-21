use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum LoginAction { Admin, User, Denied }

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    login_action: LoginAction,
}

impl User {
    fn new(username: &str, password: &str, login_action: LoginAction) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
            login_action,
        }
    }
}

fn main() {
    let users = vec![
        User::new("admin", "password", LoginAction::Admin),
        User::new("herbert", "password", LoginAction::User),
        User::new("bad", "actor", LoginAction::Denied),
    ];
    let serialized = serde_json::to_string(&users).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: Vec<User> = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);
}
