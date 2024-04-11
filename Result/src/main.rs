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
fn multipy(a_str:&str,b_str:&str)-> Result<i32,ParseIntError>{
    let a_num=a_str.parse::<i32>()?;
    let b_num=b_str.parse::<i32>()?;
    Ok(a_num * b_num)
    }
    fn main(){
        println!("{:?}",multipy("4","7").unwrap())
    }