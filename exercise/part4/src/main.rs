fn main() {
    let mut city_names: Vec<&str> = vec!["Pythonia", "Javasburg", "C by the Sea", "Rustville"];

    let mut last_city = "";

     match city_names.pop(){
        Some(value)=>{
            last_city=value;
        }
        None=>{
            println!("vector is empty");
        }
     }


   
 
    if last_city.starts_with("R") {
        println!("“{}” starts with an R!", last_city);
    } else {
        println!("“{}” doesn't start with R", last_city);
    }

    // 👉 TODO now that we've done that, use `.push()` to put last_city
    //    back in `city_names`.
    city_names.push(last_city);

    println!("Here is the full list of cities:");
    for city in city_names.iter(){
        println!("{}/n",city);
    }
}