use std::num::ParseIntError;
// fn divide(x:f32,y:f32)->Result<f32,& 'static str>{
// if x==0.0{
//     return Err("Division By zero");
// }
// Ok(x/y)
// }
// fn main(){
// let result=divide(5.5,0.0);
// match result{
//     Ok(value)=>println!("{}",value),
//     Err(msg)=>println!("{}",msg)
// }
// }
// fn multipy(a_str:&str,b_str:&str)-> Result<i32,ParseIntError>{
//     let a_num=a_str.parse::<i32>()?;
//     let b_num=b_str.parse::<i32>()?;
//     Ok(a_num * b_num)
//     }
//     fn main(){
//         println!("{:?}",multipy("4","7").unwrap())
//     }
// FILL in the blanks and FIX the errors
//using unwrap()
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}