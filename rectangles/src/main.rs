
// with structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(15);

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Rectangle is: {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}

// with tuples

fn tuple_main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        tuple_area(rect1)
    );
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


// original - no tuples

fn original_main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        original_area(width, height)
    );
}

fn original_area(width: u32, height: u32) -> u32 {
    width * height
}
