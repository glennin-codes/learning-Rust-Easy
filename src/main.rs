// // fn main(){
// //   let mut years:[i32;3]=[2001,2005,2010];
// //   years[0]=2020;

// //   // for year in years.iter(){
// //   //   println!("year:{}",year + year);
// //   // }
// // let [x,..]=years;
// // println!("value is{}",x);
// // }
// //tuples
// // fn main(){
// //   let mut  foo:(u32,bool,char)=(2882,true,'G');
// // foo.0=900;
// //   println!("my tuple is {:?}",foo);

use std::fmt::format;

// // }
// //structs
// struct Student{
//   age:u16,
//   name:String,
//   id:String,
// }
// impl Student {
//   pub fn new_student(age:u16,name:String,id:String)->Self{
//     Self{
//       age,
//       name,
//       id
//     }
//   }
//   fn get_age(&self)->u16{
//     self.age
//   }
    
// }
// fn main(){
  
// let student_a = Student::new_student(21, "glen ayienda".to_string(), "janme3434".to_string());
//   println!("student age is {}",student_a.get_age());
// }
//todo exercise 
struct City {
  description: String,
  residents: u64,
  is_coastal:bool

  // 💡 HINT: this will cause other compiler errors.
  //    They will tell you what other changes need to happen!
}

fn new_city(residents: u64, is_coastal: bool) -> City {
  if is_coastal {
      City {
          description: format!("a *coastal* city of approximately {} residents", residents),
          residents,
          is_coastal
      }
  } else {
      City{
        description:format!("*non-coastal* city of approximately {} residents",residents),
        residents,
        is_coastal
      }
  }
}

fn main() {
  let rustville: City =new_city(800, false);

  println!("This city can be described as: 👉{} ",rustville.description);

  if rustville.is_coastal {
      println!("It is a coastal city.");
  } else {
      println!("It is not a coastal city.");
  }
}