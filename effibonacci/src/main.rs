use std::io;


fn main() {

    loop{
        let mut nth: String = String::new();

        println!("Enter a nth Fibonacci number what you wish:");

        io::stdin().read_line(&mut nth)
            .expect("Something went wrong when reading line.");

        if nth.trim() == "quit" {
            break;
        }

        let nth: u128 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You enter not valid number!");
                continue
            }

        };

        let fib: u128 = get_fibonacci_number(nth);
        println!("Fibonacci number is: {}", fib)
    }
    println!("Good bye!")

}

fn get_fibonacci_number(x: u128) -> u128 {
    let mut fib_vector: Vec<u128> = Vec::new();

    for number in 0..x + 1 {
        fib_vector.push( match number {
            0 => 0,
            1 => 1,
            _ => fib_vector[number as usize - 1] + fib_vector[number as usize - 2]
        });
    }
    fib_vector[x as usize]
}
