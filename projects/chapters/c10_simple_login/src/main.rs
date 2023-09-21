fn is_login_valid(username: &str, password: &str) -> bool {
    username == "admin" && password == "password"
}

fn main() {
    println!("Login for admin/password is {}", is_login_valid("admin", "password"));
    println!("Login for admin/12345 is {}", is_login_valid("admin", "12345"));
}
