use std::convert::TryFrom;
use std::convert::TryInto;
#[derive(Debug,PartialEq)]
struct EvenNum(i32);
 impl TryFrom <i32> for EvenNum{
    type Error=();
    fn try_from(value:i32)->Result<Self,Self::Error>{
        if value % 2 == 0 {
          Ok(EvenNum(value))
        }else{
          Err(())
        }
    }
 }
 
fn main (){
 assert_eq!(EvenNum::try_from(8),Ok(EvenNum(8)));
 assert_eq!(EvenNum::try_from(5),Err(()));
 let result:Result<EvenNum,()>=32i32.try_into();
 assert_eq!(result,Ok(EvenNum(32)));
 let result:Result<EvenNum,()>=5_i32.try_into();
 assert_eq!(result,Err(()));
 println!("succes");
}