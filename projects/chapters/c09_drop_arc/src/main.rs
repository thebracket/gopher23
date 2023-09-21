use std::rc::Rc;

struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn greet(s: Rc<MyStruct>) {
    println!("Hello, {}", s.name);
}

fn main() {
    let my_struct = Rc::new(MyStruct { name: "Hello".to_string() });
    greet(my_struct.clone());
    greet(my_struct.clone());
    println!("Exiting main function");
}
