// fn main() {
//     let mut  data:[i32;5] = [1, 2, 3, 4, 5];

//     // Create a slice containing elements from index 1 to 3 (inclusive).
//     let slice = &mut data[1..=3];

//     // Print the elements of the slice.
//     for element in slice.iter_mut() {//iter_mut provides mutable references to the elements allowing you to modify them in place 
//        *element*= 2;   // Multiply each element
//     }
//     print!("slice:{:?}",slice);
// }
// //why we didnt use iter() method -it provides immutable references to the elements 
// //deferencing is done inside the for loop (*element)
// //-why?
// // //-its because when you iterate over a slice ,you get a reference to the elements ,not the elements themselves .So inorder to access and modify the actual element you need to deference the actual elements .r():iter_mut() provides mutable 
// fn main() {
//     let data:[i32;5] = [1, 2, 3, 4, 5];

//     // Create a slice containing elements from index 1 to 3 (inclusive).
//     let slice:&[i32] = &data[1..=3];

//     println!("slice: {:?}",slice);
// }