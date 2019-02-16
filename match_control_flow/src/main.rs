
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl Coin {
    fn value_in_cents(&self) -> u8{
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }
}

fn main() {
    let c: Coin = Coin::Penny;
    let nc: Coin = Coin::Quarter(UsState::Alaska);
    let mut cents: Option<u8> = Some(nc.value_in_cents());
    println!("{}", c.value_in_cents());
    println!("{}", nc.value_in_cents());
    println!("{:?}", plus_one(cents).unwrap());
    let some_value = 5u8;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("Wrong!")
    }
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}