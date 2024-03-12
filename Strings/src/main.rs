fn main() {
    let fruits:Vec<&str> = vec!["Apple", "Banana", "Orange","Mango","Pineapple" ,"Grapefruit"];
    //lets use map
   
//    let datas= fruits.iter().map(|fruit:  &&str| {
    
//     let fruit = fruit.to_string();
//     fruit
//    }).collect::<Vec<String>>();
//    println!("{:?}",datas)
// lets use filter
   let filtered=fruits.iter().filter(|fruit|
    {
        fruit.contains("a")
    }
  
).collect::<Vec<&&str>>();
println!("{:?}",filtered);
// lets use find
let found_data=fruits.iter().find(|item|{
    item.contains("a")
});
let key =found_data.unwrap();//use unwrap if your sure your data returns a value
println!("not unwraped {:?}",found_data);
println!("{:?}",key);

}
