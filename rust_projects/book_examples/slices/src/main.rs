fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let word_string: &str = &s[0..5];
    println!("The first word has length: {word} and the value is {word_string}");
    s.clear();    
}