use std::io;

fn main() {
    loop {
        println!("Choose option:");
        println!("1: Fahrenheit to Celsius");
        println!("2: Celsius to Fahrenheit");
        println!("3: Quit");

        let mut option = String::new();

        io::stdin().read_line(&mut option)
            .expect("Failed to read line");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if option == 1 {
            convert_temperature(&option);
            break;
        } else if option == 2 {
            convert_temperature(&option);
            break;
        } else if option == 3 {
            break;
        } 
    }
}

fn convert_temperature(option: &u8) {
    let mut temperature: (f64, char) = (32.0, 'F');
    let new_temperature: (f64, char);

    loop {
        println!("Please enter temperature:");

        let mut temperature_string = String::new();

        io::stdin().read_line(&mut temperature_string)
            .expect("Failed to read line");

        temperature.0 = match temperature_string.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if *option == 1 {
            temperature.1 = 'F';
            new_temperature = ((temperature.0 - 32.0) * 5.0 / 9.0, 'C');
            break;
        } else if *option == 2 {
            temperature.1 = 'C';
            new_temperature = (temperature.0 * 9.0 / 5.0 + 32.0, 'F');
            break;
        }
    }

    println!("{}°{} = {}°{}.",
             temperature.0, temperature.1,
             new_temperature.0, new_temperature.1);
}

