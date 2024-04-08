struct MyType {
    // Some resources or state that need cleanup
    data: String,
}

impl Drop for MyType {
    fn drop(&mut self) {
        // Cleanup actions go here
        println!("Dropping MyType with data: {}", self.data);
    }
}

fn main() {
    let my_value = MyType {
        data: String::from("Some data"),
    };
    println!("{:?}",my_value.data)

    // At the end of this scope, `my_value` goes out of scope and its destructor is called automatically
}
