
pub fn dealocate_memory() -> u16 {
    let numbers: Vec<u16> = vec![1223, 2343, 4563, 7803];
    let mut total:u16 = 0;
    for number in numbers.iter() {
        total = total + number;
    }
   
    total    
}

fn main() {
    println!("Total: {}", dealocate_memory());
}
