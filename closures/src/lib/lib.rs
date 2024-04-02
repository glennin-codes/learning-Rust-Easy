fn main() {
    let x = Box::new(5);

    let ..; //update only this line

    *y = 4;

    assert_eq!(*x, 5);
    assert_eq!(y, 4);

    println!("Success!");
}
