fn main() {
//    let s: &str = "hello";
    let mut s: String = String::from("hello");
    s.push_str(", universe");
    println!("{}", s);
    let x: u8 = 6;
    let y: u8 = x;
    make_a_copy(y);
    println!("{}", y);
    println!("{}", x);
    let mut ns: String = String::from("hello");
    let mut nns: String = ns.clone();
    nns.push_str("text");
    takes_ownership(&mut ns);
    println!("{}", nns);
    let ship: String = gives_ownership();
    println!("{}", ship);
    let val: String = String::from("raw value");
    let returned_val:String = takes_and_gives_back_ownership(val);
    let (strg, l) = calculate_length(returned_val);
    println!("{}, len is {}", strg, l)
}

fn takes_ownership(str0: &mut String) {
    str0.push_str("owner");
    println!("{}", str0);
}

fn make_a_copy(x: u8) {
    println!("{}", x+4)
}

fn gives_ownership() -> String {
    let ship: String = String::from("Shipping string");
    ship
}

fn takes_and_gives_back_ownership(val: String) -> String{
    val
}

fn calculate_length(raw: String) -> (String, usize) {
    let length: usize = raw.len();
    (raw, length)
}