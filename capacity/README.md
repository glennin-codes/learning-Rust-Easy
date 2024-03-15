# Capacity
- amount of memory allocated for a collection or data structure.it might be  string or a vector ``Vec<T>``

- length /Size :is the number of ellements in a container.
- A Capacity is always greater than the length or equal to the length.
-If a length of the ellements in the collection exceed the capacity when inserting new ellements ,The vector will need to allocate more memory to accomodate the new element,which could involve reallocation and copying of excisting elements.

### Necessity of Capacity Management:

- In many cases, you don't need to explicitly manage capacity, as Rust's standard library implementations handle it for you. When you append elements to a vector (Vec<T>), for example, Rust automatically manages the capacity and reallocates memory as needed to accommodate the new elements.
- However, in certain scenarios where performance is critical and you have specific knowledge about the size requirements of your data, you may choose to preallocate memory by specifying an initial capacity for the collection. This can avoid unnecessary reallocations and improve performance.

## examples
- without capacity managment
```rust
fn main(){
    let mut v:Vec<i32>=Vec::new();

for i in 0..100{
    v.push(i)
}
v.push(30);
println!("capacity:{},length:{}",v.capacity(),v.len());

 }
 ```
 output: `` capacity:128,length:101``

- with capacity mnagement

```rust
//Capacity
 fn main(){
    let mut v:Vec<i32>=Vec::with_capacity(101);

for i in 0..100{
    v.push(i)
}
v.push(30);
println!("capacity:{},length:{}",v.capacity(),v.len());

 }
```
output: `` capacity:101,length:101 ``