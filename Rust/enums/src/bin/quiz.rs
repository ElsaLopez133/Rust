/// Makes a string to separate lines of text, 
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}

/// Gets the string out of an option if it exists,
/// returning a default otherwise
fn get_or_default(arg: Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}


fn main() {
    let v = make_separator("");
    println!("The value is  {v}");

    let some_string = Some(String::from("Hello, Rust!"));
    let result1 = get_or_default(some_string);
    println!("{result1}")
}
