fn main() {
    let mut s = String::from("hello world");
    
    let word = first_word(&s);

    s.clear();

    println!("The first word in '{}' is '{}'.", s, word);
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

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if start == 0 && item == b' ' {
            start = i + 1;
        } else if item == b' ' {
            end = i;
            break;
        }
    }

    &s[start..end]
}

