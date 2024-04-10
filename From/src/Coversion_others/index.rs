use std::fmt::{ Formatter,Display, Result};
struct Point{
    x:i32,
    y:i32
}
impl  Display for Point{
  fn fmt(&self,f:&mut Formatter) ->  Result  {
    write!(f, "({},{})",self.x,self.y)
  }
}
fn main(){
    let origin:Point=Point{
        x:0,y:9
      };
      assert_eq!(origin.to_string(),"point is (0,9)");
      assert_eq!(format!("{}",origin),"(0,9)");
      println!("success");
}