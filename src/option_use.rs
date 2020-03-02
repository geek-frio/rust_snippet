fn return_option(name: &str) -> Option<&str> {
    match name {
        "a" => Some("b"),
        "c" => None,
        _ => None
    }
}

pub fn run(){
    let s = String::from("xxx");
    println!("Character index at 5:{}", match s.chars().nth(5) {
        Some(c) => c.to_string(),
        None => String::from("none")    
    });
    println!("{}", match return_option("a") {
        Some(a) => a,
        None => "no operation"
    });
}