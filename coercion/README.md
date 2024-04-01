# Coercion in Rust: Using `as` for Type Conversions

## Introduction

Coercion in Rust is a powerful feature that allows the compiler to automatically convert one type to another under certain conditions. This document focuses on the use of the `as` keyword for explicit type conversions, providing a clear understanding of how and when to use `as` for type conversions in Rust.

## Basic Usage of `as` for Type Conversion

The `as` keyword is used for explicit type conversions between primitive types, such as integers, floating-point numbers, and characters. Here's a basic example:

```rust 
fn main() { 
let x = 5; let y = x as f64;
 // Convert an i32 to an f64 
 println!("x as f64: {}", y);
  }```


In this example, `x` is an `i32` integer, and we're converting it to an `f64` floating-point number using the `as` keyword.

## Numeric Conversions with `as`

The `as` keyword can be used for converting between numeric types, including integers and floating-point numbers. However, it's important to note that using `as` for numeric conversions can lead to loss of precision or data if the target type cannot represent the value of the source type accurately.

```rust
 fn main() {
     let x = 1000; let y = x as f32; 
 // Convert an i32 to an f32 
 println!("x as f32: {}", y); }
```


In this example, `x` is an `i32` integer, and we're converting it to an `f32` floating-point number. This conversion is safe, but keep in mind that not all numeric conversions are safe, especially when converting from a larger type to a smaller one, which can lead to data loss.

## Pointer Conversions with `as`

The `as` keyword can also be used for converting between raw pointers of different types. This is useful in low-level programming where you need to work directly with memory addresses.

rust fn main() { let x = 5; let y = &x as *const i32; // Convert a reference to an i32 to a raw pointer println!("y: {:?}", y); }


In this example, `x` is an `i32` integer, and we're converting a reference to `x` into a raw pointer of type `*const i32`.

## Limitations and Safety

While the `as` keyword provides a way to perform explicit type conversions, it's important to use it judiciously. Not all conversions are safe, and using `as` can lead to undefined behavior if the conversion is not valid. For example, converting a floating-point number to an integer using `as` will truncate the fractional part, which might not be the desired behavior in all cases.

## Conclusion

Understanding the use of the `as` keyword for type conversions in Rust is crucial for writing safe and efficient code. This document provides a basic overview of how and when to use `as` for type conversions, highlighting the importance of using this feature judiciously to avoid potential pitfalls.

---

### using from/into to convert values
```rust
struct Number {
    value: i32,
}
impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self {
            value,
        }
    }
}
fn main() {
 
    //using from/into
    let number: Number = Number::from(22);
    println!("value is {}", number.value);
    let number: Number = (34_i32).into();
    println!("value is {}", number.value);
}

```
```rust
fn main(){
  let s:String=String::from("i love my girl");
  //or 
  let s:String="i love my girl".into()
}


```

**Note:** This document is intended to serve as a basic introduction to the topic. For more in-depth information, please refer to the [Rust documentation](https://doc.rust-lang.org/book/ch03-02-data-types.html#type-casting).
