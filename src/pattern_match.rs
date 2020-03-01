pub fn run() {
    let number = 2;
    match number {
        1 => println!("it is one"),
        2 => println!("it is two"),
        // 3..9 => println!("2 to 8"),
        10 | 11 => println!("It is either 10 or 11"),
        _ => println!("it does not match"),
    }

    let s = "asdf";
    match s {
        "asdf" => println!("asdf"),
        _ => {}
    }

}
