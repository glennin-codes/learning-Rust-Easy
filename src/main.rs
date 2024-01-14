struct Rectangle{
width:u32,
height:u32
}
 impl Rectangle{
    fn area(&self)->u32{
          self.width* self.height
    }
 }

fn main() {
    let r1:Rectangle=Rectangle { width:50000, height: 80000};
    println!("area of rectangle R1={}",r1.area());
}
