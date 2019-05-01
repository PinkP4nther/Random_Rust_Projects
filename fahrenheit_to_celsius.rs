use std::io::{self, Write};
use std::process;

fn main() {
    // Formula: (64F - 32) * 5/9 = temp C

    let mut degrees = String::new();
    print!("Input Degress Fahrenheit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut degrees).unwrap();

    let di: f64 = match degrees.trim().parse()
    {
        Ok(deg) => deg,
        Err(_) => {
            println!("Not an integer!");
            process::exit(0);
        }
    };

    println!("Fahrenheit: {:.2}",di as f64);
    println!("Celsius: {:.2}",(di - (32 as f64)) * (5 as f64 / 9 as f64));
}
