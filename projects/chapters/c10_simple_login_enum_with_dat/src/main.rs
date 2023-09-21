#[derive(Debug)]
enum LoginResult {
    Success { is_admin: bool },
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
    println!("Login for admin/password is {:?}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {:?}", is_login_valid("admin", "12345"));
    println!("Login for herbert/password is {:?}", is_login_valid("herbert", "password"));
    println!("Login for herbert/12345 is {:?}", is_login_valid("herbert", "12345"));
}
