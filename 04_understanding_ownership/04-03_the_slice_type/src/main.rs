fn main() {
    let s = String::from("hello world");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}

