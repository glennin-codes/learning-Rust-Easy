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
// fn main(){
//   let mut  foo:(u32,bool,char)=(2882,true,'G');
// foo.0=900;
//   println!("my tuple is {:?}",foo);

// }
//structs
struct Student{
  age:u16,
  name:String,
  id:String,
}
impl Student {
  pub fn new_student(age:u16,name:String,id:String)->Self{
    Self{
      age,
      name,
      id
    }
  }
  fn get_age(&self)->u16{
    self.age
  }
    
}
fn main(){
  
let student_a = Student::new_student(21, "glen ayienda".to_string(), "janme3434".to_string());
  println!("student age is {}",student_a.get_age());
}