
// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: std::fmt::Debug + PartialOrd+ PartialEq> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug,PartialOrd,PartialEq)]
struct Unit(i32);

fn main() {
    // let pair = Pair{
    //     x: Unit(1),
    //     y: Unit(3)
    // };
let pair:Pair<Unit>=Pair::new(Unit(9),Unit(8));

    pair.cmp_display();
}