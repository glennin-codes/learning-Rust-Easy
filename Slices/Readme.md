# Slices
- is a contaguous sequence pf elements in a collection such as an array or vector.
-provide an option of referencing the original collection without copying the data.

## has the bellow Types
1. Full slice: refers to all the elements  of a collection.``[..]  ``
2. Empty slice : refers to no element , it's length will be zero. `` []``
3. Slice with start and end index``[start..end]``
4. Slice with start but without end index ,``[start..]``


#### &[T] and  &mut [T]: 
- &[T] : It represents an  array or vector that has been borrowed as read  only(immutable).
 - &mut [T]: This represents an array   or vector which has been borrowed as mutable(read write).
  
```rust
fn main() {
    let data:[i32;5] = [1, 2, 3, 4, 5];

    // Create a slice containing elements from index 1 to 3 (inclusive).
    let slice:&[i32] = &data[1..=3];  

    println!("slice: {:?}",slice);
}

Output:```slice: [2, 3, 4]```



```rust
fn main() {
    let mut  data:[i32;5] = [1, 2, 3, 4, 5];

    // Create a slice containing elements from index 1 to 3 (inclusive).
    let slice:&mut [i32] = &mut data[1..=3];

    // Print the elements of the slice.
    for element in slice.iter_mut() {//iter_mut provides mutable references to the elements allowing you to modify them in place 
       *element*= 2;   // Multiply each element
    }
    print!("slice:{:?}",slice);
}

```
Output:`` slice: [2,4,6]``

- why we didnt use iter() method instead?
- iter_mut() method  provides mutable references to the elements 
-while iter() provides   immutable references to the elements

####  Deferencing
- Deferencing is done inside the for loop (*element)
- why?
- its because when you iterate over a slice ,you get a reference to the elements ,not the elements themselves .So inorder to access and modify the actual element you need to deference the actual elements 


 
