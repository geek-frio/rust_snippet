use std::collections::HashMap;

pub fn run() {
    let mut marks = HashMap::new();
    //
    marks.insert("a", 96);
    marks.insert("b", 97);
    marks.insert("c", 100);
    println!("How many subjects:{}", marks.len());
    match marks.get("Web Developmen") {
        Some(mark) => println!("You got mark:{}", mark),
        None => println!("You didn't study"),
    }

    //Remove
    marks.remove("a");
    for (key, value) in &marks {
        println!("key:{}", key);
        println!("value:{}", value);
    }

    //Check for value
    println!("contains:{}", marks.contains_key("a"));
}
