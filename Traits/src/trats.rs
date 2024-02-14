//implement traits
trait Hello{
    fn say_hi(&self)->String{
        String::from("hi")
    }
    fn say_something(&self)->String;

    
}
struct Student {}
impl Hello for Student{
    fn say_something(&self)->String{
        String::from("Hi am a good student")
    }
}
struct  Teacher {}
impl Hello for Teacher {
    fn say_hi(&self)->String{
        String::from("Hi ,am a new teacher")
    }
    fn say_something(&self)->String {
        String::from("I'm not a bad teacher")   }
}

fn main() {
    let s:Student=Student{};
    assert_eq!(s.say_hi(),"hi");
    assert_eq!(s.say_something(),"Hi am a good student");
 let t:Teacher=Teacher{};
    assert_eq!(t.say_hi(),"Hi ,am a new teacher");
    assert_eq!(t.say_something(),"I'm not a bad teacher");
    println!("success!")
}
