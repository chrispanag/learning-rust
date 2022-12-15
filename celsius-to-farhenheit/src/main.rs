use std::char;
use std::io::Write;
use std::io::{self, Read};

const MAX_BUFFER: usize = 1000;

#[derive(PartialEq)]
enum ReadResult {
    Exit = 1,
    Success = 0,
    CharsInNumber = 2,
    EmptyString = 3,
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn read_buffer(x: &mut String, buffer: &[u8; MAX_BUFFER]) -> ReadResult {
    for (i, uc) in buffer.iter().enumerate() {
        let c = *uc as char;
        match c {
            '\n' => {
                if x.len() > 0 {
                    *x = String::from(x.trim());
                    return ReadResult::Success;
                }

                return ReadResult::EmptyString;
            }
            'e' => return ReadResult::Exit,
            '-' => {
                if i == 0 {
                    String::push(x, c);
                    continue;
                }

                return ReadResult::CharsInNumber;
            }
            '.' => String::push(x, c),
            '0'..='9' => String::push(x, c),
            _ => return ReadResult::CharsInNumber,
        }
    }

    ReadResult::EmptyString
}

fn read_string(x: &mut String) -> Result<ReadResult, &str> {
    let mut buffer: [u8; MAX_BUFFER] = [0; MAX_BUFFER];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(_) => return Ok(read_buffer(x, &buffer)),
            Err(_) => {
                return Err("Read failure!");
            }
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("Input a temperature in celsius...\ncelsius = ");
        io::stdout().flush().expect("print flush");

        let mut x = String::new();
        match read_string(&mut x).expect("read_string") {
            ReadResult::Exit => {
                println!();
                break;
            }
            ReadResult::Success => {
                println!();

                let x: f64 = x.parse().expect("parse_string");

                let fahrenheit = celsius_to_fahrenheit(x);

                println!("{x} celsius is {fahrenheit} fahrenheit");
            }
            ReadResult::CharsInNumber => {
                println!();
                println!("Chars in number!");
            }
            ReadResult::EmptyString => {
                println!();
            }
        }

        println!();
    }

    println!("Exiting...");
    Ok(())
}
