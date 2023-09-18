# Serialization

Rust is really good at serialization and de-serialization. Using a crate named `serde` (pronounced "ser-dee" according to the author, I tend to say "serd"), you can easily add serialization support to your types.

```rust
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
    let serialized = serde_json::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);
}
```

Serde isn't just for JSON! Serde has support---through different crates---for a lot of different formats. Here's the same thing with YAML:

```rust
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
    let serialized = serde_yaml::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = serde_yaml::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);
}
```

Or TOML:

```rust
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
    let serialized = toml::to_string(&user).unwrap();
    println!("Serialized:\n {}", serialized);
    let deserialized: User = toml::from_str(&serialized).unwrap();
    println!("Deserialized: {:#?}", deserialized);
}
```

Serde works with collections, nested types (anything that also support `Serialize`/`Deserialize`):

```rust
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
```

Serde offers a lot of options. You can decorate fields with [attributes](https://serde.rs/field-attrs.html) to give you more control. You can rename fields, alias them, skip fields, flatten nested structures, and even define zero-copy borrowing to field data that matches the representation.

Serde is also not just for text. You can use `cbor`, `bincode` and others to serialize to binary formats. You can also define your own serialization and deserialization functions instead of deriving them.