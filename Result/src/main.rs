// // use std::num::ParseIntError;
// // fn divide(x:f32,y:f32)->Result<f32,& 'static str>{
// // if x==0.0{
// //     return Err("Division By zero");
// // }
// // Ok(x/y)
// // }
// // fn main(){
// // let result=divide(5.5,0.0);
// // match result{
// //     Ok(value)=>println!("{}",value),
// //     Err(msg)=>println!("{}",msg)
// // }
// // }
// // fn multipy(a_str:&str,b_str:&str)-> Result<i32,ParseIntError>{
// //     let a_num=a_str.parse::<i32>()?;
// //     let b_num=b_str.parse::<i32>()?;
// //     Ok(a_num * b_num)
// //     }
// //     fn main(){
// //         println!("{:?}",multipy("4","7").unwrap())
// //     }
// // FILL in the blanks and FIX the errors
// //using unwrap()
// // fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
// //     let n1 = n1_str.parse::<i32>();
// //     let n2 = n2_str.parse::<i32>();
// //     Ok(n1.unwrap() * n2.unwrap())
// // }

// // fn main() {
// //     let result = multiply("10", "2");
// //     assert_eq!(result, Ok(20));

// //     let result = multiply("4", "2");
// //     assert_eq!(result.unwrap(), 8);

// //     println!("Success!");
// // }

// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f = File::open("README.md");
//     let mut f:File = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
// println!("{:?}",f);
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(b) =>{
//             println!("number of bytes read is {}",b);
//             Ok(s)
//         },
//         Err(e) =>  Err(e),
//     }
    
// }

// // FILL in the blanks with one code line
// // DON'T change any code lines
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

// File::open("README.md")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     // assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!");
//    match read_file1(){
//     Ok(s)=>println!("string {}",s),
//     Err(e)=>println!("error {}",e)
//    }
// }
use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
// But it's so Verbose...
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1)  => {
//             match n2_str.parse::<i32>() {
//                 Ok(n2)  => {
//                     Ok(n1 * n2)
//                 },
//                 Err(e) => Err(e),
//             }
//         },
//         Err(e) => Err(e),
//     }
// }

// Rewriting `multiply` to make it succinct
// You should use BOTH of  `and_then` and `map` here.
// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     // IMPLEMENT.../
//     n1_str.parse::<i32>().and_then(|n1| n2_str.parse::<i32>().map(|n2|n1 * n2))
// }

// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
fn get_value(x: i32) -> Result<i32, &'static str> {
    if x > 5 {
        Ok(x)
    } else {
        Err("Value is too small")
    }
}

fn multiply(x: i32) -> Result<i32, &'static str> {
    Ok(x * 2)
}


fn main() {
    // // This still presents a reasonable answer.
    // let twenty = multiply1("10", "2");
    // print(twenty);

    // // The following now provides a much more helpful error message.
    // let tt = multiply("t", "2");
    // print(tt);

    // println!("Success!");
    let x: Result<i32, &str> = Ok(9);
let y: Result<i32, &str> = x.and_then(|v|get_value(v)).and_then(|v|multiply(v)); // y is Ok(16)

// let x: Result<i32, &str> = Ok(3);
// let y: Result<i32, &str> = x.and_then(|v|get_value(v)).and_then(|v|multiply(v)); // y 

    if let Ok(v)=y{
        println!("{}",v)
    }else{
        println!("Got an Error: {:?}",y.unwrap_err());
    }
}