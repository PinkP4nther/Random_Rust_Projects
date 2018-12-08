use std::io::{self,Write};

fn main() {

    let full_name: String = String::from("Pink Panther");
    let mut name = String::new();

    prompter("Guess my name (first last): ".to_string(),&mut name);
    
    {
    let iter = &mut name.split_whitespace();
    
    let fname = match iter.next() {
        Some(ref x) => x,
        None => "No First Name!",
    };
    println!("First Name: {}",fname);

    let lname = match iter.next() {
        Some(ref x) => x,
        None => "No Last Name!",
    };
    println!("Last Name: {}",lname);
    }

    if name.trim() == full_name {
        println!("[+] Nice Job! You guessed my name!");
    } else {
        println!("[-] Nice try, but incorrect");
    }

    println!("You guessed {}",name);

    println!("Without vowels, that's {}",strip_vowels(&mut name).trim());

}

fn prompter(p: String, b: &mut String) {
    print!("[?] {}",p);
    io::stdout().flush().unwrap();
    io::stdin().read_line(b).unwrap();
}

fn strip_vowels(s: &mut String) -> String{
    
    let vowels: [&str;10] = ["a","e","i","o","u","A","E","I","O","U"];
    let mut ret_string = String::from("");
    for ss in s.split_whitespace() {
        let mut tmp_app = String::new();
        let mut tmpmutstr = String::from(ss);
        for i in 0..=10 {
            if i <= 9 {
                tmpmutstr = tmpmutstr.replace(vowels[i],"").to_string();
            }
            else {
                tmp_app.push_str(tmpmutstr.as_str());
            }
        }
        ret_string.push_str(tmp_app.as_str());
        ret_string.push_str(" ");
    }
    ret_string
}
