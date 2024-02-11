use std::usize;
use std::ops::AddAssign;
#[derive(Debug)]

struct Array<T,const N:usize>{
    data:[T;N]
}

impl<T, const N: usize> Array<T, N> 
where
    T: AddAssign<T> + Default,
{
    fn iterator(self) -> T {
        let mut total: T = Default::default();
        for data in self.data {
            total += data;
        }
        total
    }
}
fn main() {
   let students:Array<f64,4>=Array{
    data:[7.8,4.0,5.9,8.1]
   };
  

   print!("the student position  is {}", students.iterator());

}
