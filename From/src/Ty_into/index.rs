use std::convert::TryInto;
    

fn main(){
     let n:i16=1000;
     let n :u8= match n.try_into(){
        Ok(n)=>n,
        Err(e)=>{
            println!("{}",e.to_string());
            0
        }
     };
     println!("value {}",n);

}