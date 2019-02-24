struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct UserInfo<T, U> {
    user: T,
    numeric_country_code: U,
}

impl <T, U> UserInfo <T, U> {
    fn mixup<V, W>(self, other: UserInfo<V, W>) -> UserInfo<T, W> {
        UserInfo {
            user: self.user,
            numeric_country_code: other.numeric_country_code,
        }
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

fn main() {

    let integer_point = Point { x: 3, y: 6};
    let float_point: Point<f32> = Point { x: 3.0, y: 6.0};
    println!("{}", float_point.x);
    println!("{}", float_point.distance_from_origin());

    let user = UserInfo {user: "Michael".to_string(), numeric_country_code: 113};
    let second_user = UserInfo {
        user: "Sarah".to_string(),
        numeric_country_code: "211".to_string()
    };
    let mixed_user = user.mixup(second_user);
    dbg!(mixed_user.user);
    dbg!(mixed_user.numeric_country_code);
    let none_option: MyOption<i32> = MyOption::None;
    let some_option = MyOption::Some("text example".to_string());

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
    let char_list = vec!['a', 'k', 'z', 'm', 'o'];

    let largest_char = largest_char(&char_list);
    println!("Largest char is: {}", &largest_char);
//    println!("(Generic) Largest char is: {}", generic_largest(&char_list));
//    println!("(Generic) Largest number is: {}", generic_largest(&second_number_list));


}

fn generic_largest<T>(list: &[T]) -> T {
    let mut largest_number = list[0];

    for &item in list.iter() {
        if item > largest_number {
            largest_number = item;
        }
    }
    largest_number
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

fn largest_char(list: &[char]) -> char {
    let mut largest_number = list[0];

    for &item in list.iter() {
        if item > largest_number {
            largest_number = item;
        }
    }
    largest_number
}

