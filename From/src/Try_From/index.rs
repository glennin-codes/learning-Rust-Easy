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
 assert_eq!(EvenNum::TryFrom(8),Ok(EvenNum(8)));
 assert_eq!(EvenNum::TryFrom(5),Err(EvenNum(())));
 let result:Result<EvenNum,()>=i32.try_into();
 assert_eq!(Result,Ok(EvenNum(32)));
 let result:Result<EvenNum,()>=i5.try_into();
 assert_eq!(Result,Err(()));
}