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
    fn vic(){
        let n:i16=56;
        let n=n as u8;
        // let n:u8=match n.try_into(){
        //   Ok(n)=>n,
        //   Err(e)=>{println!("the was an error while converting,{:?}",e.to_string());
        //   0}

        // };
        println!("{}",n)
    
       }
       vic();
    //using from/into
    let number: Number = Number::from(22);
    println!("value is {}", number.value);
    let number: Number = (34_i32).into();
   
    println!("value is {} ", number.value);

}
