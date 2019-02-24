use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let f= File::open("hello.txt");

    let _fl = match f {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem : {:?}", e),
            },
            _other_error => panic!("There was a problem opening the file: {:?}", error),
        },
    };

    let _new_file = File::open("hello_again.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_again.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem : {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });

    let _unwrap_file = File::open("hello.txt").unwrap();
//    let _expect_file = File::open("hi.txt").expect("Failed to open hi.txt");
    let _error = read_username_from_file();
    dbg!(_error);
    let _normal_error = normal_read_username_from_file();
    dbg!(_normal_error);
    let _short_error = short_read_username_from_file();
    dbg!(_short_error);
    let _even_shorter_error = even_shorter_read_username_from_file();
    dbg!(_even_shorter_error);
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hi.txt");

    let mut f  = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn normal_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hi.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hi.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hi.txt")
}