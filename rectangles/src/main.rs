#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width, height));

    let rect = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect));

    let rect = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_struct(&rect));
    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}