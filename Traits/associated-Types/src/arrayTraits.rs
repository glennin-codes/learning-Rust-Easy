trait Bird{
    fn quack(&self);
}
struct Duck;
impl Duck{
    fn fly(&self){
        println!("look this duck can fly!")
    }
}
struct Swan;
impl Swan{
    fn fly(&self){
        println!("the Swan is flying")
    }
}
impl Bird for Duck{
    fn quack(&self){
        println!("Duck Quack")
    }
}
impl Bird for Swan {
    fn quack(&self){
        println!("Swan Quack")
    }
}fn main()->(){
    let birds:[&dyn Bird;2]=[&Duck,&Swan];
    for bird in birds.iter(){
        bird.quack();
    }
}