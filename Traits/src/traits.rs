
use std::ops;

pub fn multiply<T>(a: T, b: T) -> T 
where
T:ops::Mul<Output = T>
{
    a * b
}

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(2.2f32,multiply(1.0f32,2.2f32));
    println!("succes!");
}
