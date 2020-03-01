use std::io;

pub fn run(){
    let mut input = String::new();
    println!("Hey mate! Say something:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success, you said:{}", input);
        },
        Err(e) => {
            println!("Oops, something went wrong:{}", e);
        }
    }
}