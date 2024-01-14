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
    // let r1:Rectangle=Rectangle { width:50000, height: 80000};
    // println!("area of rectangle R1={}",r1.area());
    let light:TrafficLight=TrafficLight::new();

    assert_eq!(light.color,"red");
    assert_eq!(light.get_state(),"red");
    println!("success")

}
// impliment associationm. they dont take self
struct TrafficLight{
    color:String

}
impl TrafficLight{
    pub fn new()->Self{
        Self{
            color:"red".to_string()
        }
    
    }
    
    pub fn get_state(&self)->&str{
        &self.color
    }
}
