# Understanding Rust's unwrap() Method

The unwrap() method in Rust is a convenient tool for extracting values from Option and Result types. However, it's essential to use it judiciously, as it can lead to panics if used incorrectly.

Usage:

1.Option:

When dealing with an Option, unwrap() extracts the inner value if it exists.
```rust
let some_value = Some(5);
let unwrapped_value = some_value.unwrap(); // unwrapped_value will be 5
However, if the Option is None, unwrap() will panic.
rust
Copy code
let none_value: Option<i32> = None;
let unwrapped_none = none_value.unwrap(); // This will panic
```
2.Result:

Similarly, with a Result, unwrap() retrieves the inner value if it's an Ok.
```rust
let result: Result<i32, &str> = Ok(42);
let unwrapped_result = result.unwrap(); // unwrapped_result will be 42
```
But if the Result is an Err, unwrap() will cause a panic.

```rust
let err_result: Result<i32, &str> = Err("Error message");
let unwrapped_err = err_result.unwrap(); // This will panic

```

   #Strings
###A string has a pointer ,a length and a capacity
####capacity is the amount of memory allocated to the string
####length is the amount of memory currently used by the string
####the length should be less than or equal to the capacity
####if the length is greater than the capacity the string will be reallocated
####this is done by doubling the capacity of the string
####realocation is so expensive so its better to use the with_capacity method to allocate the memory in advance

```rust
fn main(){
  let mut  s:String=String::with_capacity(34);
  println!("{}",s.capacity());
  for _ in 0..3{
    s.push_str("hello world");
    println!("{}",s.capacity());

  }
  println!("length of the string is {}",s.len())
}
```
The output of this program would be :
```rust
34
34
34
34
length of the string is 33
```

