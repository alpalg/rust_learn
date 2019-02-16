#[derive(Debug)]

struct Rectangle {
    width: u16,
    height: u16,
}


fn main() {
    let width: u16 = 32;
    let height: u16 = 32;

    println!("Area equal to {}.", area(width, height));
    let rect: (u16, u16) = (32,32);


    println!("Area equal to {}.", rearea(rect));
    let str_rect: Rectangle = Rectangle {
        width: 26,
        height: 42,
    };

    println!("Area equal to {}.", sarea(&str_rect));
    println!("Area equal to {:#?}.", str_rect);
}

fn area(w: u16, h: u16) -> u16 {
    w * h
}

fn rearea(dimensions: (u16, u16)) -> u16 {
    dimensions.0 * dimensions.1
}

fn sarea(rect: &Rectangle) -> u16 {
    rect.width * rect.height
}