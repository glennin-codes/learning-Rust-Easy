//Capacity
 fn main(){
    let mut v:Vec<i32>=Vec::with_capacity(101);

for i in 0..100{
    v.push(i)
}
v.push(30);
println!("capacity:{},length:{}",v.capacity(),v.len());

 }
 