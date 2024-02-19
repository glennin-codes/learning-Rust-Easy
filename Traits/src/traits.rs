use std::ops;


fn multiply<T:ops::Mul<Output = T>>(a:T,b:T)->T{
    a*b
}
pub fn main(){
   assert_eq!(6,multiply(2u8,3u8));
   assert_eq!(4.0,multiply(2.0f32,2.0f32));
   println!("success!");
}
