pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    if age > 21 && check_id {
        println!("..");
    } else if age < 21 && check_id {
        println!("...");
    } else {
        println!(" I will need see your id");
    }

    // shorthand if 
    let is_of_age = if age >= 21 { true } else {false};
    println!("Is of age:{}", is_of_age);
}
