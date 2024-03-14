//vectors like array but can grow and shrink
//allocated on the heap as contagous block of memory
//use when you have a list of items and you don't know how many items you will have at compile time. 
// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v:Vec<u8> = Vec::from(arr);
//     is_vec(&v);

//     let v:Vec<u8> = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) and vec![..] are same macros, so
//     let v:Vec<u8> = vec!(1, 2, 3);
//     is_vec(&v);
    
//      // In code below, v is Vec<[u8; 3]> , not Vec<u8>
//     // USE Vec::new and `for` to rewrite the below code 
//     let mut  v1:Vec<u8> = Vec::new();
//   v.iter().for_each(|v: &u8|{
//         v1.push(*v)
//     });
//     is_vec(&v1);
 
//     assert_eq!(v, v1);

    
   
//     println!("Success! {:?}",v1);
// }

// fn is_vec(_v: &Vec<u8>) {}
///
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.extend(&v1);
//     assert_eq!(v1, v2);

//     println!("Success!");
// }
fn main(){
    let s:String="hello world".to_string();
    let v:Vec<u8>=Vec::from(s);

//turn a string into a vector of bytes Vec<u8>
    let s:String="hello world".to_string();
    let v2:Vec<u8>=s.into_bytes();
    assert_eq!(v,v2);

    
   
//turn a string of &str into Vec<u8>

    let s:&str= "hello world";
    let v3:Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);
    println!("Success,{:?}",v);

  }