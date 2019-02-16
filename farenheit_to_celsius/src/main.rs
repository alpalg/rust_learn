use std::io;

fn main() {
    let mut farenheit: String = String::new();

    println!("Please enter a Farenheit grades:\n");
    io::stdin().read_line(&mut farenheit)
        .expect("Something is wrong.");

    let farenheit:u16 = match farenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Value was set to 0 because you enter not valid number.");
            0
        }
    };

    println!("Grades in Farenheit: {}.", farenheit);
    let celsius: f32 = (farenheit as f32 - 32.0) * 5.0/9.0;

    println!("Grades in Celsius: {}.", celsius);

}
