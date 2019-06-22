fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut count = 0;

    while count != 12 {
        println!("On the {} {}", days[count], "day of Christmas");
        println!("My true love gave to me");
        if count > 10 { println!("Twelve drummers drumming") };
        if count > 9 { println!("Eleven pipers piping") };
        if count > 8 { println!("Ten lords a-leaping") };
        if count > 7 { println!("Nine ladies dancing") };
        if count > 6 { println!("Eight maids a-milking") };
        if count > 5 { println!("Seven swans a-swimming") };
        if count > 4 { println!("Six Geese a-laying") };
        if count > 3 { println!("Five gold rings") };
        if count > 2 { println!("Four calling birds") };
        if count > 1 { println!("Three french hens") };
        if count > 0 { println!("Two turtle doves") };
        println!("A partridge in a pear tree\n");
        count += 1;
    }
}
