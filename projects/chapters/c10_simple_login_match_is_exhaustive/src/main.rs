#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
    LockedOut,
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    match (username, password) {
        ("admin", "password") => LoginResult::Success { is_admin: true },
        ("herbert", "password") => LoginResult::Success { is_admin: false },
        (_, _) => LoginResult::Failure,
    }
}

fn main() {
    match is_login_valid("herbert", "password") {
        LoginResult::Success { is_admin: true } => {
            println!("Hello, admin!");
        }
        LoginResult::Success { is_admin: false } => {
            println!("Hello, user!");
        }
        LoginResult::Failure => {
            println!("Login failed.");
        }
        // Comment this out to see the error:
        LoginResult::LockedOut => {
            println!("Your account is locked out.");
        }
    }
}
