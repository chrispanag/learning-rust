use std::char;
use std::io::Write;
use std::io::{self, Read};

const MAX_BUFFER: usize = 1000;

#[derive(PartialEq)]
enum ReadResult {
    Exit = 1,
    Success = 0,
    Failure = -1,
    CharsInNumber = -2,
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn read_string(x: &mut String) -> ReadResult {
    let mut buffer: [u8; MAX_BUFFER] = [0; MAX_BUFFER];
    let mut res = ReadResult::Failure;
    'outer: loop {
        match io::stdin().read(&mut buffer) {
            Ok(_) => {
                for i in buffer {
                    let c = i as char;
                    match c {
                        c if char::is_numeric(c) => String::push(x, c),
                        '.' => String::push(x, c),
                        '\n' => {
                            if x.len() > 0 {
                                res = ReadResult::Success;
                                break 'outer;
                            } else {
                                print!("celsius = ");
                                match io::stdout().flush() {
                                    Ok(_) => {
                                        continue;
                                    }
                                    Err(_) => {
                                        break;
                                    }
                                }
                            }
                        }
                        'e' => return ReadResult::Exit,
                        _ => return ReadResult::CharsInNumber,
                    }
                }
            }
            Err(_) => {
                println!("Read failure!");
                break;
            }
        }
    }

    *x = String::from(x.trim());
    res
}

fn main() -> io::Result<()> {
    loop {
        print!("Input a temperature in celsius...\ncelsius = ");
        io::stdout().flush()?;
        let mut x = String::new();
        let res = read_string(&mut x);
        match res {
            ReadResult::Exit => {
                println!("Exiting...");
                break;
            }
            ReadResult::Failure => {
                println!("Panic!");
                break;
            }
            ReadResult::Success => {
                println!();

                let x: f64 = x
                    .parse()
                    .expect("Error, parse_string should only return if number exists");

                let fahrenheit = celsius_to_fahrenheit(x);

                println!("{x} celsius is {fahrenheit} fahrenheit");
                println!();
                continue;
            }
            ReadResult::CharsInNumber => {
                println!("Chars in number!");
                println!();
                continue;
            }
        }
    }

    Ok(())
}
