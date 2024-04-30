pub fn example2 (){

}
pub  fn example1() {
    let mut  v=Vec::new();
    for n in 0..100{
        v.push(n)
    }
    assert_eq!( v.len(),100);
   println!("{:#?}",v);

}
