use std::char;
use std::io::Write;
use std::io::{self, Read};

const MAX_BUFFER: usize = 1000;

#[derive(PartialEq)]
enum ReadResult {
    Exit = 1,
    Success = 0,
    CharsInNumber = -2,
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * (9.0 / 5.0) + 32.0
}

fn read_string(x: &mut String) -> Result<ReadResult, &str> {
    let mut buffer: [u8; MAX_BUFFER] = [0; MAX_BUFFER];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(_) => {
                for i in buffer {
                    let c = i as char;
                    match c {
                        c if char::is_numeric(c) => String::push(x, c),
                        '.' => String::push(x, c),
                        '\n' => {
                            if x.len() > 0 {
                                *x = String::from(x.trim());
                                return Ok(ReadResult::Success);
                            } else {
                                print!("celsius = ");
                                match io::stdout().flush() {
                                    Ok(_) => {
                                        continue;
                                    }
                                    Err(_) => return Err("Error flushing"),
                                }
                            }
                        }
                        'e' => return Ok(ReadResult::Exit),
                        _ => return Ok(ReadResult::CharsInNumber),
                    }
                }
            }
            Err(_) => {
                return Err("Read failure!");
            }
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("Input a temperature in celsius...\ncelsius = ");
        io::stdout().flush()?;
        let mut x = String::new();
        let res = read_string(&mut x);
        match res {
            Ok(res) => match res {
                ReadResult::Exit => {
                    println!("Exiting...");
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
            },
            Err(err) => {
                println!("{err}");
                break;
            }
        }
    }

    Ok(())
}
