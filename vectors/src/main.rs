enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let v: Vec<u16> = Vec::new();
    let mut v: Vec<u8> =vec![1,2,3];
    dbg!(&v);
    v.push(4);
    v.push(5);
    v.push(7);
    dbg!(&v);
    for item in &v {
        println!("Item #{}", item);
    }

    for item in &mut v {
        *item += 8;
    }
    dbg!(& v);
    let third: &u8 = &v[2];
    dbg!(&third);

    match v.get(2) {
        Some(third) => println!("{}", third),
        None => println!("There is no third element.")
    }

    let does_not_exist: Option<&u8> = v.get(10);
    dbg!(does_not_exist);
    // let does_not_exist: &u8 = &v[10]; Don`t work

    let mut row = vec![
        SpreadsheetCell::Int(15),
        SpreadsheetCell::Float(2.3),
        SpreadsheetCell::Text(String::from("field_text"))
    ];

    let val = row.pop();

    // Broken part
    // Because mut and immut references was created
//    let mut v1 = vec![1,2];
//
//    let first = &v1[0]; //immut
//
//    v1.push(3); // mut
//    println!("{}", first);
}
