struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            value,
        }
    }
}
fn main() {
 
    //using from/into
    let number: Number = Number::from(22);
    println!("value is {}", number.value);
    let number: Number = (34_i32).into();
   
    println!("value is {} ", number.value);
}
