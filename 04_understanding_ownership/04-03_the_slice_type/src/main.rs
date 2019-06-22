fn main() {
    let some_string = String::from("hello world");
    let word = first_word(&some_string);
    println!("The first word in '{}' is '{}'.", some_string, word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
