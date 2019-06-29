#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle{ width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.height * self.width 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let square1 = Rectangle::square(100);

    println!("square1 is {:#?}", square1);
}

