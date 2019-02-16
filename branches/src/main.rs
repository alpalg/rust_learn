fn main() {
    let number: u8 = 7;

    let condition: bool = false;

    let number: u8 = if condition {
        number * 2
    }else {
        number
    };

    if number < 5 {
        println!("Condition was true. {}", number);
    }else {
        println!("Condition was false. {}", number);
    }

    if number == 0 {
        println!("Number was something other than zero.")
    }

    if number % 4 == 0 {
        println!("Number is divisible by 4.")
    }else if number % 3 == 0 {
        println!("Number is divisible by 3.")
    }else if number % 2 == 0 {
        println!("Number is divisible by 2.")
    }else {
        println!("Number is not divisible by 4, 3 or 2.")
    }

    let mut counter: u8 = 0;

    let result: u8 = loop {
        counter += 1;
        println!("forever");
        if counter == 5{
            break counter - 4;
        }
    };

    assert_eq!(result, 1);

    while counter != 15 {
        println!("counter is {}.", counter);
        counter += 1;
    }

    counter = 0;
    let values_list: [u8; 5] = [10, 20, 30, 40, 50];

//    let values_count: u8 = ;
    let mut counter: usize = counter as usize;
    while counter  < values_list.len() {
        println!("Value is {}.", values_list[counter]);
        counter += 1;
    }

    for element in values_list.iter() {
        println!("Element is {}.", element);
    }

    for numb in 1..6 {
        println!("{}", numb)
    }

        for numb in (1..6).rev() {
        println!("{}", numb)
    }
}

