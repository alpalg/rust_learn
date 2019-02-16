fn main() {
    let s1:String = String::from("hallo");
    let len: usize = calculate_length(&s1);
    println!("String: `{}` and it's length is {}.", s1, len);
    let mut s2:String = String::from("hallo");
    append_post(&mut s2);
    println!("{}", s2);
    let double_word: String = String::from("Deeps space");
    let ind: &str = get_first_word(&double_word);
    println!("{}", ind);
//    println!("{}", &double_word[..=4]);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_post(s: &mut String){
    s.push_str("das Welt");
}

fn get_first_word(s: &str) -> &str {
    let bytes:&[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}