#[derive(Debug)]
enum LoginResult {
    Success,
    Failure,
}

fn is_login_valid(username: &str, password: &str) -> LoginResult {
    if username == "admin" && password == "password" {
        LoginResult::Success
    } else {
        LoginResult::Failure
    }
}

fn main() {
    println!("Login for admin/password is {:?}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {:?}", is_login_valid("admin", "12345"));
}
