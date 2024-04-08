// fn main() {
//     let x = 5;

//     // Define a closure that captures the variable x
//     let print_x = || {
//         println!("x is: {}", x);
//     };

//     // Call the closure
//     print_x();
// }

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

// fn main() {
//     let x = 7;
//     let y = 9;
    
//     print_one(&x);
//     print_multi(&x, &y);
    
//     let z = pass_x(&x, &y);
//     print_one(z);

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }fn outer_function(outer_variable: &str) -> impl Fn(&str) {
    fn outer_function <'a> (outer_variable: &'a str) -> impl Fn(&'a str) {
        move |inner_variable: &str| {
            // Inner function logic using outer_variable and inner_variable
            println!("Outer variable: {}", outer_variable);
            println!("Inner variable: {}", inner_variable);
        }
    }
    fn outer_function_b(outer_variable: i32) -> impl Fn(i32) -> i32 {
        let closure = move |inner_variable: i32| {
            outer_variable + inner_variable
        };
        closure
    }
    use std::ptr; 
    fn main() {
        // Create a closure by calling the outer function and assigning it to a variable
        let closure = outer_function("outerValue");
     let outer_var = 10;
        let closure = outer_function_b(outer_var);
        let result = closure(5);
        println!("Result: {}", result);
        // Access the inner function and use it
        // closure("innerValue");
        add_one(&mut 8);
        let x = Box::new(5);
    
        let mut  y = *x;
        
        y = 4;
        
        assert_eq!(*x, 5);
        assert_eq!(y, 4);
    
        println!("Success!");
        referencing();
        practice();
    }
    fn add_one<'a>(x: &'a mut i32)->() {//life time of x is same as the life time of the reference passed to it 
        *x += 1;
        println!("{}",x);
    }

fn referencing() {
    let c: char = '中';

    let r1: &char = &c;
    let ref  r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
    
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}",ptr::addr_of!(r))
}
/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn practice() {
    let mut count = 0;

    let mut inc = move  || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    let _reborrow = &count;

    inc();
   
    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count; 
  
    assert_eq!(count, 0);
   
}