# Hashamps
- Is a data structure that maps keys to values.
- provides fast look up ,insertion and deletion operations with an average time complexity of 0(1)
## examples

```rust
 let mut scores: HashMap<&str, i32>=HashMap::new();
    scores.insert("oranges", 3);
    scores.insert("mangoes", 4);
    scores.insert("guavas", 3);





```
#### Getting a value
- it returns an option
```rust
//getting a value
let  get_value: Option<&i32>=scores.get("oranges");
// using if let
if let Some(value)=get_value {
    println!("The value of mangoes is : {:?}",value);
}else {
    println!("The value of mangoes is not found");
}

```
#### removing a value
```rust
 //removing a value 
    scores.remove("oranges");
```
#### iterating through the hashmap
```rust
//iterating through the hashmap
for (key,value) in scores{
    println!("key:{} value:{}",key,value);
}
```



