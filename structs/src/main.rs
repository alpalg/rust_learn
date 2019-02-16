#![allow(unused_variables)]

struct User {
    username: String,
    email: String,
    password: String,
    is_active: bool,
}

struct Point(u8, u8, u8);

fn main() {
    let mut new_user: User = new_user("newbie",
                                      "123456",
                                      "newbie@gmail.com");
    new_user.is_active = false;
    let newest_user: User = User {
        ..new_user
    };
//    println!("{}", new_user.email);
    println!("{}", newest_user.username);
    let null_point: Point = Point(5,6,7);
    println!("{}", null_point.1)
}


fn new_user(username: &str, password: &str, email: &str) -> User {
    User {
        username: String::from(username),
        password: String::from(password),
        email: String::from(email),
        is_active: true,
    }
}