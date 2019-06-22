#![allow(unused_variables)]
fn main() {
    let reference = no_dangle();
}

fn no_dangle() -> String { 
    let s = String::from("hello"); // s is a new String

    s
} 

