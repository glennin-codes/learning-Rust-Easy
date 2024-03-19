#[derive(Debug, PartialEq)]

//you can use enums to pass data of different types in a vector 
//this is useful when you want to pass data of different types in a vector
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}