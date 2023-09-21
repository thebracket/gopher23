struct MyStruct {
    name: String,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {name}", name=self.name);
    }
}

fn main() {
    let _my_struct = MyStruct { name: "Hello".to_string() };
    println!("Exiting Main Function");
}