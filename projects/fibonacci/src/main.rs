fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Generate the nth Fibonacci number!");
    println!("Please choose n: ");

    let number: u128 = loop {
        let mut number = String::new();

        std::io::stdin().read_line(&mut number)
            .expect("Failed to read line.");

        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    println!("Fibonacci number: {}", fibonacci(number));
}
