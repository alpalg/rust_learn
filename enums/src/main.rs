#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

//struct IpAddress {
//    kind: IpAddressKind,
//    address: String,
//}

enum Message {
    Quit,
    Move {x: u32, y: u32},
    Write(String),
    ChangeColor(u16, u16, u16),
}

impl Message {
    fn call(&self) {
        println!("Hids");
    }
}

fn main() {
    let home: IpAddress = IpAddress::V4(127, 0, 0, 1);
    let loopback: IpAddress = IpAddress::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    let msg: Message = Message::Write(String::from("Hi Jack!!!"));
    msg.call()
}
