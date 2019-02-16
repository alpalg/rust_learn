#[derive(Debug)]
enum Coin {
    Quarter(String),
    Penny,
    Dime,
}


fn main() {
    let some_value = Some(3u8);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("if let three")
    }

    let mut coin: Coin = Coin::Quarter(String::from("Alaska"));
    let mut count: u32 = 0;
    match &coin {
        Coin::Quarter(state) => println!("State of quarter from {:?}", state),
        _ => count += 1,
    };
    if let Coin::Quarter(state) = &coin {
        println!("State of quarter from {:?}", state)
    }else {
        count += 1
    };

}
