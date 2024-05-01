
pub  fn example1() {
    let mut  v=Vec::new();
    for n in 0..100{
        v.push(n)
    }
    assert_eq!( v.len(),100);
   
}
pub fn example2 (){
    let arr=vec![0;10];
    for i in arr.iter(){
       println!("{}",i);
    }
    println!("{:?}",arr);
   }
pub fn  example3(){
let mut  names:Vec<&str>=vec!["Lamaphosa","Chuvick","Glen"];
for  name in names.iter_mut(){
    *name= match name{
       &mut "Glen"=> "There is a Rustatcean among us!",
      _=>"hello"
    }

}
println!("{:#?}",names);

}
pub fn example4 ()->(){
let mut values:Vec<i32>=vec![1,2,3];
let mut values_iter_mut=values.iter_mut();
    if let Some(v)=values_iter_mut.next(){
       
       *v=0;
       println!("v:{}",v);
    }else{
       println!("no elements");
    }
   assert_eq!(values,vec![0,2,3]);
}
//implimenting our own iterator
pub fn example5(){
    #[derive(Debug)]
    struct Counter{
        count:u32
    }
    impl Counter{
        fn new()->Counter{
            Counter{
                count:0
            }
        }
    }
    impl Iterator for Counter{
        type  Item=u32;
        fn next(&mut self)->Option<Self::Item>{
           if self.count < 5{
               self.count+=1;
               Some(self.count)
           } else{
            None
           }
        }
    }
    let mut count_values: Counter=Counter::new();
    count_values.next();
    count_values.next();
    count_values.next();
    count_values.next();
    count_values.next();
    count_values.next();
    count_values.next();
    println!("{:?}",count_values);

}