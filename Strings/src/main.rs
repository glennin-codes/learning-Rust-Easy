// fn main() {
//     let fruits:Vec<&str> = vec!["Apple", "Banana", "Orange","Mango","Pineapple" ,"Grapefruit"];
//     //lets use map
   
// //    let datas= fruits.iter().map(|fruit:  &&str| {
    
// //     let fruit = fruit.to_string();
// //     fruit
// //    }).collect::<Vec<String>>();
// //    println!("{:?}",datas)
// // lets use filter
//    let filtered=fruits.iter().filter(|fruit|
//     {
//         fruit.contains("a")
//     }
  
// ).collect::<Vec<&&str>>();
// println!("{:?}",filtered);
// // lets use find
// let found_data=fruits.iter().find(|item|{
//     item.contains("a")
// });
// let key =found_data.unwrap();//use unwrap if your sure your data returns a value
// println!("not unwraped {:?}",found_data);
// println!("{:?}",key);

// }

//strings
//a string has a pointer ,a length and a capacity
//capacity is the amount of memory allocated to the string
//length is the amount of memory currently used by the string
//the length should be less than or equal to the capacity
//if the length is greater than the capacity the string will be reallocated
//this is done by doubling the capacity of the string
//realocation is so expensive so its better to use the with_capacity method to allocate the memory in advance


fn main(){
  let mut  s:String=String::with_capacity(34);
  println!("{}",s.capacity());
  for _ in 0..3{
    s.push_str("hello world");
    println!("{}",s.capacity());

  }
  println!("legth of the string is {}",s.len())
}