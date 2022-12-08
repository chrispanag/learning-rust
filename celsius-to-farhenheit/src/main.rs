use std::char;
use std::io::Write;
use std::io::{self, Read};

const BACKSPACE: char = 8u8 as char;
const MAX_BUFFER: usize = 1000;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn parse_string() -> String {
    let mut x = String::new();
    let mut buffer: [u8; 1000] = [0; MAX_BUFFER];
    'outer: loop {
        let l = io::stdin().read(&mut buffer).expect("Failed to read bytes");
        if l > MAX_BUFFER {
            println!("Buffer overflow!");
            return String::from("EXIT");
        }
        for i in 0..=l {
            let c = buffer[i] as char;
            if char::is_numeric(c) || c == '.' {
                String::push(&mut x, c);
                continue;
            }
            if c == '\n' {
                if x.len() > 0 {
                    break 'outer;
                } else {
                    print!("celsius = ");
                    io::stdout().flush().expect("Issue flushing!");
                    continue;
                }
            }
            print!("{}", BACKSPACE);
            if c == 'e' {
                return String::from("EXIT");
            }
        }
    }

    String::from(x.trim())
}

fn main() -> io::Result<()> {
    loop {
        print!("Input a temperature in celsius...\ncelsius = ");
        io::stdout().flush()?;
        let x = parse_string();

        println!();

        let x: f64 = match x.parse() {
            Ok(num) => num,
            Err(_) => {
                if x == "EXIT" {
                    println!("Exiting...");
                    break;
                } else {
                    println!("Invalid character, please input number");
                    continue;
                }
            }
        };

        let fahrenheit = celsius_to_fahrenheit(x);

        println!("{x} celsius is {fahrenheit} fahrenheit");
        println!()
    }

    Ok(())
}
