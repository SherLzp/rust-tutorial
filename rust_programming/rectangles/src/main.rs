#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rec1 = Rectangle { length: 50, width: 30 };
    println!(
        "The area of the rectangle is {} square pixels",
        rec1.area()
    );
}
