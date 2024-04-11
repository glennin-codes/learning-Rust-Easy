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