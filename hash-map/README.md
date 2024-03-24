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

### use or_insert("value") or or_insert_with()
- Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
- ``or_insert() `` its a function that takes a concrete value as its parameter.
- ``or_insert_with() ``  takes a closure that generates the default value.

```rust
let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);


```
- In terms of usage, if you already know the default value you want to insert, or_insert() is simpler and more straightforward. However, if the default value requires computation or you want to ensure that it's lazily evaluated only when necessary, or_insert_with() is more appropriate because it allows you to pass a closure that calculates the default value.
```rust 
let mut player_stats = HashMap::new();

    // Insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert_with(random_value);
    fn random_value ->u8 (){
        // Could actually return some random value here - let's just return
    // some fixed value for now
    26
    }
```

