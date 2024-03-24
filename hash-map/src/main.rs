// use::std::collections::HashMap;
// use std::ptr::null;



// fn main(){
//     let mut scores: HashMap<&str, i32>=HashMap::new();
//     scores.insert("oranges", 3);
//     scores.insert("mangoes", 4);
//     scores.insert("guavas", 3);

//     println!("{:?}",scores);
//     //removing a value 
//     scores.remove("oranges");
//     println!("After removing oranges : {:?}",scores);
// //getting a value
// let  get_value: Option<&i32>=scores.get("oranges");
// //using if let
// // if let Some(value)=get_value {
// //     println!("The value of mangoes is : {:?}",value);
// // }else {
// //     println!("The value of mangoes is not found");
// // }

// //using match

// // match get_value{
// //     Some(value)=>println!("The value of mangoes is : {:?}",value),
// //     None=>println!("The value of mangoes is not found")
// // }

// //using unwrap
// // - unwrap method is used to get the value of the key from the hashmap if your sure the value is present in the hashmap
// // - if the value is not present in the hashmap it will panic
// // println!("The value of mangoes is : {:?}",get_value.unwrap());
// //iterating through the hashmap
// for (key,value) in scores{
//     println!("key:{} value:{}",key,value);
// }
// }
// FILL in the blanks
use std::collections::HashMap;
fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // Insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    // Ensures a value is in the entry by inserting the default if empty, and returns
    // a mutable reference to the value in the entry.
    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

fn random_stat_buff() -> u8 {
    // Could actually return some random value here - let's just return
    // some fixed value for now
    42
}