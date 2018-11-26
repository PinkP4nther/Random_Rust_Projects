use std::io::{self,Write};
use std::process;

fn main() {

    println!("[ Calculator (Rust Practice) ]");

    loop {
        println!("\n[Menu]\n  [1] => Add\n  [2] => Subtract\n  [3] => Divide\n  [4] => Multiply\n  [5] => Exit\n");
        let mut choice = String::new();
        let mut ci: u32;
        loop {
            print!("[?] Select option: ");
            io::stdout().flush().unwrap();

            // Get input for choice
            io::stdin().read_line(&mut choice).unwrap();

            // Make sure Input is convertable to unsigned int
            ci = match choice.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {
                    choice = String::new(); // Renew String Type Var
                    continue;
                } // If not continue loop from beginning
            };

            // Check u32 that its in correct range of options
            if ci < 6 && ci > 0 {
                break;
            }
            else
            {
                println!("[!] Choice must be between 1 and 5");
            }
        }

        if ci == 1
        {
            let (x,y): (f64,f64) = get_vals("add".to_string());
            println!("[Sum] {} + {} = {}",x,y,add(x,y));
        }
        else if ci == 2
        {
            let (x,y): (f64,f64) = get_vals("subtract".to_string());
            println!("[Difference] {} - {} = {}",x,y,subtract(x,y));
        }
        else if ci == 3
        {
            let (x,y): (f64,f64) = get_vals("divide".to_string());
            println!("[Quotient] {} / {} = {}",x,y,divide(x,y));
        }
        else if ci == 4
        {
            let (x,y): (f64,f64) = get_vals("multiply".to_string());
            println!("[Product] {} * {} = {}",x,y,multiply(x,y));
        }
        else if ci == 5
        {
            println!("[!] Exiting!");
            process::exit(0x0);
        }
        else
        {
            println!("[-] Ruh roh this shouldn't have happened!");
        }
    } 
}

fn get_vals(mode: String) -> (f64,f64) {

    let val1: f64  = loop{
        let mut val = String::new();
        print!("[{}] Val1: ",mode);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut val).unwrap();
        match val.trim().parse()
        {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };// End loop
    
    let val2: f64 = loop{
        let mut val = String::new();
        print!("[{}] Val2: ",mode);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut val).unwrap();
        match val.trim().parse()
        {
            Ok(num) => break num,
            Err(_) => continue,
        };
    }; //End loop

    (val1,val2)
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

fn divide(x: f64, y: f64) -> f64 {
    x / y
}

fn multiply(x: f64, y: f64) -> f64 {
    x * y
}
