# Vectors

vectors like array but can grow and shrink

allocated on the heap as contagous block of memory

use when you have a list of items and you don't know how many items yo

```rust
  special macro:vec!
  ```
  ### Basic example
  ```rust
  fn main() {
    let arr: [u8; 3] = [1, 2, 3];
    
    let v:Vec<u8> = Vec::from(arr);// from changes ownership to vec 
    is_vec(&v);

    let v:Vec<u8> = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v:Vec<u8> = vec!(1, 2, 3);
    is_vec(&v);
    
     // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code 
    let mut  v1:Vec<u8> = Vec::new();//new creates an empty array
  v.iter().for_each(|v: &u8|{    //for_each  takes in a closure that returns nothing (void function) but  it does take one argument and performs iteration 
        v1.push(*v)
    });
    is_vec(&v1);
 
    assert_eq!(v, v1);

    
   
    println!("Success! {:?}",v1);
}

fn is_vec(_v: &Vec<u8>) {}
```
## 🌟🌟 A Vec can be extended with extend method
- It takes iterator that yields values of type T

```rust // FILL in the blank
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend();
    assert_eq!(v1, v2);

    println!("Success!");
}
```
### from arr to Vec
```rust 
fn main(){
  // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr:[i32;3] = [1, 2, 3];
    let v1 :Vec<i32>= Vec::from(arr);
    
    let v2: Vec<i32> =arr.into();
    assert_eq!(v1,v2);

    println!("Success");
}
```
### from String into Vec
-all strings are utf encoded and they are stored in the heap as vec<u8>
```rust
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
  ```
  ```rust
  // Indexing
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
       let value=v.get(i);
        println!("{:?}", value)
    }

    for i in 0..5 {
       match v.get(i){
           Some(e)=>v[i]=e+ 1,
           None=>v.push(i + 2
           )
       }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
  ```
## Slice


