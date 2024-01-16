fn main(){
  let years:[i32;3]=[2001,2005,2010];

  for year in years.iter(){
    println!("year:{}",year + year);
  }

}