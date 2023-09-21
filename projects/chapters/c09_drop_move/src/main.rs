struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn greet(s: MyStruct) {
    println!("Hello, {}", s.name);
}

fn main() {
    let my_struct = MyStruct { name: "Hello".to_string() };
    greet(my_struct);
    println!("Exiting main function");
}
