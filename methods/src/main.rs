#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
        self.area() > another_rectangle.area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 30,
    };
        let rect1: Rectangle = Rectangle {
        width: 21,
        height: 28,
    };
        let rect2: Rectangle = Rectangle {
        width: 40,
        height: 51,
    };

    println!("{:?}", &rect.area());
    println!("{:?}", &rect);
    println!("{:?}", &rect.can_hold(&rect1));
    println!("{:?}", &rect.can_hold(&rect2));

    let squ = Rectangle::square(60);

    println!("{:?}", &squ);

}
