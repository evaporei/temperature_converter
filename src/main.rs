use std::io;

fn main() {
    println!("Temperature converter");

    io::stdin().read_line(&mut unit)
        .expect("Failed to read line");

    loop {
        println!("Choose which unit you have: (c or C for Celsius, f or F for Fahrenheit)");

        let mut unit = String::new();

        io::stdin().read_line(&mut unit)
            .expect("Failed to read line");

        if unit != "c" && unit != "C" && unit != "f" && unit != "F" {
            continue;
        }

        loop {
            let mut temperature = String::new();

            io::stdin().read_line(&mut temperature)
                .expect("Failed to read line");

            let temperature: f32 = match temperature.trim().parse() {
                Ok(temp) => temp,
                Err(_) => continue,
            };

            let result: f32 = match unit {
                "c" | "C" => temperature * 1.8 + 32,
                "f" | "F" => (temperature - 32) * 0.5556,
            };

            println!("You've got {}º", result);
        }

        break;
    }
}
