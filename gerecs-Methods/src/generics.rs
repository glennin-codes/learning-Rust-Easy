struct Rectangle {
    width:u32,
    height:u32
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height

    }
}
fn main (){
    let rect1:Rectangle=Rectangle{height:50,width:30};
    assert_eq!(rect1.area(),1500);
    print!("success")
}