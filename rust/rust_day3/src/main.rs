mod sample;
use sample::{sample_function, get_rusty};
fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The perimeter of the rectangle is {} pixels ", rect1.perimeter());

    sample_function();   
    get_rusty() 
}

// Structs

struct Rectangle {
    width: u32,
    height: u32,
}

// area
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// perimeter

impl Rectangle {
    fn perimeter(&self) -> u32{
        self.height + self.width
    }
}

