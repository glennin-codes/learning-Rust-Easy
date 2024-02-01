fn main(){
    let mut v:String=String::from("hello,");
    let r:&mut String = &mut v;

    match r{
        value if value == "hello," =>value.push_str("world!"),
        _=>println!("unknown")
    }
println!("r:{}",r);
}