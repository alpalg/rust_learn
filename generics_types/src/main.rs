fn main() {
    let number_list = vec![12, 43, 54, 2, 4, 5, 234, 56, 322];

    let mut largest_number  = number_list[0];

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }
    println!("Largest number is: {}", &largest_number);

    let second_number_list = vec![1, 2, 26, 4, 5, 6, 13, 4, 11];

    let second_largest = largest(&second_number_list);


    println!("Largest number is: {}", &second_largest);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest_number = list[0];

    for &item in list.iter() {
        if item > largest_number {
            largest_number = item;
        }
    }
    largest_number
}