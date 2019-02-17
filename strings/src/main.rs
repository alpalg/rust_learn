fn main() {
    let s: String = String::new();

    let raw_data = "initial data";
    dbg!(raw_data);
    let s1: String = raw_data.to_string();
    dbg!(&s1);
    let s2: String = "initial data".to_string();
    dbg!(&s2);
    let hello = String::from("שָׁלוֹם");
    dbg!(&hello);
    let hello = String::from("नमस्ते");
    dbg!(&hello);
    let mut hello: String = String::from("Hi, ");
    let elle = " and Elle ";
    hello.push_str("Mike");
    hello.push_str(&elle);
    dbg!(&hello);
    let mut s3 = "lo".to_string();
    s3.push('l');
    dbg!(&s3);
    let s4: String = hello + &s3;
    dbg!(&s4);
    let s5: String = s1 + "-" + &s2 + "-" + &s3 + "-" + &s4;
    let s5_format: String = format!("{}-{}-{}", &s2, &s3, &s4);
    let rs: String = "Тестирование".to_string();
    dbg!(&s5);
    dbg!(&s5_format);
    dbg!(&s5_format[0..1]);
    dbg!(&rs[0..4]);
    for c in rs.chars() {
        dbg!(c);
    }

    for b in rs.bytes() {
        dbg!(b);
    }

}
