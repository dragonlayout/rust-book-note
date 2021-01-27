#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let width = 40;
    // let height = 50;
    // println!("The area of the rectangles is {}", area(width, height));

    // let rect1 = (10, 10);
    let rect2 = Rectangle {
        width: 10,
        height: 20
    };
    println!("The rectangles is {:#?}", rect2);
    // println!("The area of the rectangles is {}", area(&rect2));
    println!("The area of the rectangles is {}", rect2.area());
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// Method Syntax
// difference with function
// 1. methods are defined within the context of a struct(or a enum or a trait object)
// 2. first parameter is always self
