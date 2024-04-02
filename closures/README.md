# Closures
- A closure is a special type of function that can capture and store variables from its surrounding environment, even after the outer function has finished executing. In other words, a closure is like a mini-function that remembers the environment in which it was created.

### lets use a simple explanation in kavascript like a pseudocode 
- To create a closure in JavaScript, you typically define a function within another function and return the inner function. This allows the inner function to maintain access to the variables in the outer function's scope even after the outer function has finished executing. Here is the basic syntax for creating a closure in JavaScript:
```js
function outerFunction(outerVariable) {
    function innerFunction(innerVariable) {
        // inner function logic using outerVariable and innerVariable
    }
    return innerFunction;
}

// Create a closure by calling the outer function and assigning it to a variable
let closure = outerFunction(outerValue);

// Access the inner function and use it
closure(innerValue);
```
 ###  How to use closures in rust
       ```rust
       fn outer_function(outer_variable: &str) -> impl Fn(&str) {
    move |inner_variable: &str| {
        // Inner function logic using outer_variable and inner_variable
        println!("Outer variable: {}", outer_variable);
        println!("Inner variable: {}", inner_variable);
    }
}
fn outer_function_b(outer_variable: i32) -> impl Fn(i32) -> i32 {
    let closure = |inner_variable: i32| {
        outer_variable + inner_variable
    };
    closure
}

fn main() {
    // Create a closure by calling the outer function and assigning it to a variable
    let closure = outer_function("outerValue");
 let outer_var = 10;
    let closure = outer_function(outer_var);
    let result = closure(5);
    println!("Result: {}", result);
    // Access the inner function and use it
    closure("innerValue");
}

       ```                                                                                                                                        
       - ``impl Fn(&'a str)``: This part specifies that the return type of outer_function is a closure that implements the Fn trait. The trait bounds (Fn(&'a str)) indicate that the closure can be called with a reference to a string with the lifetime ``'a``.