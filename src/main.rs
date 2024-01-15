// 
//implementing enums
#[derive(Debug)]
enum TrafficLight{
    Yellow,
    Red,
    Green,
}
impl TrafficLight{
  fn color(&self)-> &str{
      match self{
        Self::Yellow=>"yellow",
        Self::Red => "red",
        Self::Green => "green",
        
        
      }
  }
}
fn main(){
    let wel: &str="hello world..";
    let valid_string="shjjhsd ".to_string()+wel + &"djksd".to_string();
    println!("testing smth:{}",valid_string);
    let c :TrafficLight=TrafficLight::Yellow;
    println!("color of enum Yellow is {}",c.color());
    println!("succes {:?}",c)
}