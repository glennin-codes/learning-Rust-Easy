trait MyTrait{
    type MyType;
    
}

struct MyStruct;
impl MyTrait for MyStruct{
    type MyType = i32;

}

fn main() {
    
    let typo:MyStruct=MyStruct;
    let my_value: <MyStruct as MyTrait>::MyType = 10;
    println!("my_value: {}", my_value);

}
