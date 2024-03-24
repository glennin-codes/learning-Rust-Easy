use::std::collections::HashMap;
use std::ptr::null;



fn main(){
    let mut scores: HashMap<&str, i32>=HashMap::new();
    scores.insert("oranges", 3);
    scores.insert("mangoes", 4);
    scores.insert("guavas", 3);

    println!("{:?}",scores);
    //removing a value 
    scores.remove("oranges");
    println!("After removing oranges : {:?}",scores);
//getting a value
let  get_value: Option<&i32>=scores.get("oranges");
//using if let
// if let Some(value)=get_value {
//     println!("The value of mangoes is : {:?}",value);
// }else {
//     println!("The value of mangoes is not found");
// }

//using match

// match get_value{
//     Some(value)=>println!("The value of mangoes is : {:?}",value),
//     None=>println!("The value of mangoes is not found")
// }

//using unwrap
// - unwrap method is used to get the value of the key from the hashmap if your sure the value is present in the hashmap
// - if the value is not present in the hashmap it will panic
// println!("The value of mangoes is : {:?}",get_value.unwrap());
//iterating through the hashmap
for (key,value) in scores{
    println!("key:{} value:{}",key,value);
}
}
