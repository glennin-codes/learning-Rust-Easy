// struct Rectangle {
//     width:u32,
//     height:u32
// }
// impl Rectangle{
//     fn area(&self)->u32{
//         self.width*self.height

//     }
// }
// fn main (){
//     let rect1:Rectangle=Rectangle{height:50,width:30};
//     assert_eq!(rect1.area(),1500);
//     print!("success")
// }
// #[derive(Debug)]
// struct BackGround{
//     color:String
// }
// impl BackGround {
//     pub fn display(&self){
//         println!("color is {:?}",self.color);
//     }
//     pub fn change_state(&mut self){
//         self.color="green".to_string();

//     }
// }
// fn main()->(){
//     let mut home_page:BackGround=BackGround{color:"yellow".to_string()};
//     home_page.display();
//     home_page.change_state();
//     home_page.display();
//     println!("success,{:?}",home_page)
// }



//Implement struct
#[derive(Debug)]
// struct Point <T>{
//     x:T,
//     y:T
// }
// fn main (){
//     let integer:Point<u32>=Point {x:15,y:9};
//     let float:Point<f32> = Point {x:1.2,y:4.4};
//     println!("point in integer{:?}",integer);
//     println!("point in float{:?}",float);
//     println!("point X is {} on the x-axis and {} on the y-axis ",float.x,float.y);
//     println!("success")
// }
//association
// struct Rectangle {
//     width:u32,
//     height:u32
// }
// impl Rectangle{
//    pub fn new()->Self{
// Self { width: 10, height: 12 }

//    }
//    pub fn get_height(&self)->u32{
//     self.height

//    }
// }
// fn main()->(){
//     let rect1:Rectangle=Rectangle::new();
// assert_eq!(rect1.get_height(),12);
// println!("success");
    
// }
//generics
//implementing generics function
// fn sum<T:std::ops::Add<Output = T>>(a:T,b:T)->T{
//   a + b
// }
// fn main ()->(){
//     assert_eq!(18,sum(8,10));
//     assert_eq!(0.017,sum(0.008,0.009));
//     assert_eq!(0.017,sum(0.008f64,0.009f64));
//     println!("sucess!");
// }

// struct Point<T,U>{
//     y:U,
//     x:T
// }
// fn main()->(){
//     let point_p:Point<f32,u32>=Point { y: 8, x: 8.99 };
//     print!("point P is {:?}",point_p)
// }
struct Point <T>{
    val_a:T
}

impl  Point{
    fn value(&self ){
        self.val_a
    }

}
fn main (){
    let x=Val{val_a:3.0};
    let y=Val{val_a:"hello".to_string()};
    println!("{},{}",x.value(),y.value());

}