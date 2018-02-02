use std::io;

fn main() {
    println!("Temperature converter");

    loop {
        println!("Choose which unit you have: (c or C for Celsius, f or F for Fahrenheit)");

        let mut unit = String::new();

        io::stdin().read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim();

        match unit {
            "c" | "C" | "f" | "F" => {
                loop {
                    println!("Type the value you want to convert");

                    let mut temperature = String::new();

                    io::stdin().read_line(&mut temperature)
                        .expect("Failed to read line");

                    let temperature: f32 = match temperature.trim().parse() {
                        Ok(temp) => temp,
                        Err(_) => continue,
                    };

                    let constant = 32.0;

                    let result: f32 = match unit {
                        "c" | "C" => temperature * 1.8 + constant,
                        "f" | "F" => (temperature - constant) * 0.5556,
                        _ => {
                            println!("Wierd you've got here");
                            0.0
                        },
                    };

                    let new_unit = match unit {
                        "c" | "C" => "F",
                        "f" | "F" => "C",
                        _ => {
                            println!("Wierd you've got here");
                            ""
                        },
                    };

                    println!("You've got {}º{}", result, new_unit);
                    break;
                }

                break;
            },
            _ => continue,
        }

    }
}
