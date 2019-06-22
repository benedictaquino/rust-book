fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without hte slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
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

