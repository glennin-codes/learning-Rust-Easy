// trait MyTrait{
//     type MyType;
    
// }

// struct MyStruct;
// impl MyTrait for MyStruct{
//     type MyType = i32;

// }

// fn main() {
    
//     let typo:MyStruct=MyStruct;
//     let my_value: <MyStruct as MyTrait>::MyType = 10;
//     println!("my_value: {}", my_value);

// }


trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self)->() {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn main() {
    // FILL in the blank.
    let duck:Duck = Duck;
    duck.swim();

    let bird:Box<dyn Bird>= hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");
  
    let bird:Box<dyn Bird>= hatch_a_bird(0);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");
}   

// IMPLEMENT this function.
fn hatch_a_bird(species:u8)->Box<dyn Bird>  {
    match species{
        1 =>Box::new(Swan),
        2 =>Box::new(Duck),
        _ =>panic!("Not implemented")
    }
}
