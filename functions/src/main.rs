fn main() {
    println!("Hello, world!");

    another_function(10, 8);

    let val: u8 = {
        let k: u8 = 7;
        k * k
    };
    println!("The K value equal to {}", val);

    let d = double(15);

    println!("The D value equal to {}", d);

}

fn another_function(x: u8, y: u8) {
    println!("The X value equal to {}", x);
    println!("The Y value equal to {}", y);
}

fn double(x: u8) -> u8 {
    x * 2
}