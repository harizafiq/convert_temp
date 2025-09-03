use std::io;

#[derive(Debug)]
enum Choice {
    CtoF,
    FtoC,
    Exit,
}

fn get_choice() -> Choice {
    loop {
        println!("Please Choose
    1. Celsius to Fahrenheit
    2. Fahrenheit to Celsius
    3. Exit
        ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line");
        let choice = choice.trim();
        return match choice {
            "1" => Choice::CtoF,
            "2" => Choice::FtoC,
            "3" => Choice::Exit,
            _ => {
                println!("Choose only 1, 2 and 3\n");
                continue;
            }
        }
    }
}

fn get_temp() -> Option<f64> {
    loop {
        println!("Enter temp to convert");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("failed to read line");
        let temp = temp.trim();
        return match temp.parse::<f64>() {
            Ok(x) => Some(x),
            Err(e) => {
                println!("Please insert number: {}", e);
                continue
            }
        }
    }
}

fn convert_c_to_f(t: f64) -> f64 {
    t * 1.8 + 32.0
}

fn convert_f_to_c(t: f64) -> f64 {
    (t - 32.0) / 1.8
}

fn main() {
    println!("Welcome to Conversion from Celsius to Fahrenheit and vice versa");
    loop {
        match get_choice() {
            Choice::CtoF => {
                let temp = get_temp();
                if let Some(x) = temp {
                    let convert_temp = convert_c_to_f(x);
                    println!("Celsius is {} convert to Fahrenheit is {}", x, convert_temp);
                } else {
                    println!("failed to read temp");
                }
            },
            Choice::FtoC => {
                let temp = get_temp();
                println!("{:?}", temp);
                if let Some(x) = temp {
                    let convert_temp = convert_f_to_c(x);
                    println!("Fahrenheit is {} convert to Celsius is {}", x, convert_temp);
                } else {
                    println!("failed to read temp");
                }
            },
            Choice::Exit => {
                println!("Bye bye");
                break;
            }
        }
        println!("");
    }
}
