// //implimentaing sort 

// fn selelection_sort(arr: &mut [i32]){
//     for i in 0..arr.len(){
//         let mut small_index= i;
//         for j in (i + 1)..arr.len() {
//             if arr[j] < arr[small_index] {
//                 small_index=j;
              
//             }
        
//         }   
//         arr.swap(i,small_index);
        

//     }
// }
// fn main() {
//     let mut arr = [71,20,33,45,40,99,234,31,8,10,0,1,22];
//   selelection_sort(&mut arr);
//   println!("sorted: {:?}",arr)
// }
// fn quicksort(arr: &mut [i32]) {
//     if arr.len() <= 1 {
//         return;
//     }

//     let pivot = arr[arr.len() / 2];
//     let (left, right) = arr.partition(|&x| x < pivot);
//     let (mut equal, right) = right.partition(|&x| x == pivot);

//     quicksort(&mut left);
//     quicksort(&mut equal);
//     arr[..left.len() + equal.len()].copy_from_slice(&left[..]);
//     arr[left.len() + equal.len()..].copy_from_slice(&equal[..]);
// }

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let reversed_arr: Vec<_> = arr.iter().rev().cloned().collect();
    println!("{:?}", reversed_arr); // Output: [5, 4, 3, 2, 1]
}