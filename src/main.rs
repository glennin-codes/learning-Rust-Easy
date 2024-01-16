// fn main(){
//   let mut years:[i32;3]=[2001,2005,2010];
//   years[0]=2020;

//   // for year in years.iter(){
//   //   println!("year:{}",year + year);
//   // }
// let [x,..]=years;
// println!("value is{}",x);
// }
//tuples
fn main(){
  let foo:(u32,bool,char)=(2882,true,'G');
  let (_x,_y,..)=foo;
  println!("my tuple is {:?}",foo);

}