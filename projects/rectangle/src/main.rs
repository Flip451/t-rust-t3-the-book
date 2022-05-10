#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(width: u32) -> Rectangle {
        Rectangle { width: width, height: width}
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    
    println!("The area of rect1 is {} square pixels.", area(&rect1));
    
    println!("The area of rect1 is {} square pixels.", rect1.area());
    println!("The area of rect1 is {} square pixels.", &rect1.area());
    println!("The area of rect1 is {} square pixels.", &&rect1.area());
    println!("The area of rect1 is {} square pixels.", &&&rect1.area());
    println!("The area of rect1 is {} square pixels.", &&&&rect1.area());
    
    println!("rect1 is {:#?}", rect1);
    println!("rect2 is {:#?}", rect2);
    println!("rect3 is {:#?}", rect3);
    
    println!("Rect1 can hold rect2.: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold rect3.: {}", rect1.can_hold(&rect3));
    
    let square1 = Rectangle::square(100);
    println!("square1 is {:#?}", square1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
