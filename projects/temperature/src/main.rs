use std::io;

fn main() {
    let mut temperature: (f64, char) = (32.0, 'F');
    let new_temperature: (f64, char);

    loop {
        println!("Please enter temperature:");

        let mut degrees = String::new();

        io::stdin().read_line(&mut degrees)
            .expect("Failed to read line");

        temperature.0 = match degrees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        println!("[F]ahrenheit or [C]elsius?");

        let mut units = String::new();

        io::stdin().read_line(&mut units)
            .expect("Failed to read line");

        temperature.1 = match units.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    new_temperature = convert_temperature(temperature);

    println!("{}°{} = {}°{}.",
             temperature.0, temperature.1,
             new_temperature.0, new_temperature.1);
}

fn convert_temperature(temperature: (f64, char)) -> (f64, char) {
    let new_temperature: (f64, char) = if temperature.1 == 'F' {
        ((temperature.0 - 32.0) * 5.0 / 9.0, 'C')
    } else if temperature.1 == 'C' {
        (temperature.0 * 9.0 / 5.0 + 32.0, 'F')
    } else {
        temperature  // return same temperature if not F or C
    };

    return new_temperature; 
}

