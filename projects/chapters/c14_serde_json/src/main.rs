use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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
    let user = User::new("herbert", "password");

    // JSON
    println!("JSON");
    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);

    // TOML
    println!("\nTOML");
    let serialized = toml::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = toml::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);

    // YAML
    println!("\nYAML");
    let serialized = serde_yaml::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = serde_yaml::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);
}
