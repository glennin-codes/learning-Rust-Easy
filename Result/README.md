# Result
Is an enum type that represents the outcome of an operation that could potentially fail.
Has to possible variants
   - Ok(T) : a value of type T was found.
   - Err(e) : an error was found of value e.
 - `Result<Ok(T),Err(e)>`

 Since Result is an Enum ,the possible variants can be matched using a ``Match pattern``
- Matching Results is called `Error handling`

```rust

fn divide(x:f32,y:f32)->Result<f32,& 'static str>{
if x==0.0{
    return Err("Division By zero");
}
Ok(x/y)
}
fn main(){
let result=divide(5.5,0.0);
match result{
    Ok(value)=>println!("{}",value),
    Err(msg)=>println!("{}",msg)
}
}
```
## the `?`
```rust
use std::num::ParseIntError;
fn multipy(a_str:&str,b_str:&str)-> Result<i32,ParseIntError>{
    let a_num=a_str.parse::<i32>()?;
    let b_num=b_str.parse::<i32>()?;
    Ok(a_num * b_num)
    }
    fn main(){
        println!("{:?}",multipy("4","7").unwrap())
    }
```
The `?` operator is used to propagate the error up the call stack. If the parse method returns an Ok variant, the `? `operator simply returns the value inside the` Ok (in this caswe T)`. However, if the operation returns an Err variant, the `? `operator returns from the current function with that error value.
## unwrap()  
- unwrap method  extract the value from the ok variant in the Result enum but an error ocurs it panics .in any operations, you have to be confidence enough that the operation will succed.

```rust
// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20);

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}
```
 
 ## Map & and_then
 In Rust, map and and_then are methods defined for the` Result` and `Option` types, which are used for handling potential errors and nullable values, respectively. Here's when to use each one:

### map

The map method is used when you want to apply a function to the value contained within a Result or Option, but only if it is an Ok or Some value. If the value is an Err or None, it is returned as is.

The general syntax for map is:

```rust
fn map<B, F>(self, f: F) -> Result<B, E>
where
    F: FnOnce(T) -> B,
```
You would typically use map when you want to transform the value inside a Result or Option into a different type or perform some operation on it, but you don't need to handle the error case explicitly.

Example:

```rust
let x: Result<i32, &str> = Ok(5);
let y: Result<i32, &str> = x.map(|v| v + 1); // y is Ok(6)

let x: Option<i32> = Some(5);
let y: Option<i32> = x.map(|v| v + 1); // y is Some(6) 

```

### and_then

The and_then method is used when you want to apply a function that returns a Result or Option to the value contained within another Result or Option. If the original value is an Err or None, it is returned as is. If the original value is an Ok or Some, the function is applied, and its result is returned.

The general syntax for and_then is:

```rust 
fn and_then<U, F>(self, f: F) -> Result<U, E>
where
    F: FnOnce(T) -> Result<U, E>,
```

You would typically use and_then when you need to perform multiple operations that can potentially fail, and you want to chain them together. It allows you to propagate errors from one operation to the next.

Example:

```rust
fn get_value(x: i32) -> Result<i32, &'static str> {
    if x > 5 {
        Ok(x)
    } else {
        Err("Value is too small")
    }
}

fn multiply(x: i32) -> Result<i32, &'static str> {
    Ok(x * 2)
}

let x: Result<i32, &str> = Ok(8);
let y: Result<i32, &str> = x.and_then(get_value).and_then(multiply); // y is Ok(16)

let x: Result<i32, &str> = Ok(3);
let y: Result<i32, &str> = x.and_then(get_value).and_then(multiply); // y is Err("Value is too small")

```

In summary, use map when you want to transform a value inside a Result or Option, and use and_then when you need to chain multiple operations that can potentially fail and propagate errors.

## Type alias
Using  std::result::Result<T, ParseIntError> everywhere is verbose and tedious, we can use alias for this purpose.

At a module level, creating aliases can be particularly helpful. Errors found in a specific module often has the same Err type, so a single alias can succinctly defined all associated Results. This is so useful even the std library supplies one: io::Result.

example;
```rust 
use std::num::ParseIntError;

// allias type 
type Res<i32> = Result<i32,ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}
```