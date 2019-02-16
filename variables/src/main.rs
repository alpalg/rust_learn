fn main() {
    let mut x: u8 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y: u8 = 5;

    let y: u8 = y + 6;

    let y: u8 = y * y;

    println!("The value of y is: {}", y);

    let hashes = "#########";
    let hashes = hashes.len();

    println!("Len of hashes is {}.", hashes);
//    let mut questions_marks: &str = "?????????????";

//    questions_marks: usize  = questions_marks.len(); // cant mutate data type
//    let mut questions_marks: usize  = questions_marks.len();
//    println!("Question marks count is {}.", questions_marks);

    let sum = 43 / 5;
    println!("Division of two int`s is {}", sum);

    let t: bool = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    let heart_eyed_cat: char = '😻';
    println!("Cat: {}", heart_eyed_cat);

    // tuple
    let tuple_obj: (i32, f64, u8) = (50000, 3.431212323, 15);
    println!("{}, {}, {}", tuple_obj.0, tuple_obj.1, tuple_obj.2);

//    let (q,w,e) = tuple_obj;
    let (_,w,_) = tuple_obj;
    println!("{}", w);

    // array (fixed size)
    let array: [u8; 5] = [1,2,3,4,5];

    println!("{}, {}, {}, {}, {}", array[1], array[2], array[3], array[4], array[0])



}
