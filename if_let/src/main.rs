fn main() {
    let some_value = Some(3u8);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
