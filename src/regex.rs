extern crate regex;

use regex::Regex;

pub fn run() {
    // let re = Regex::new(r"\d");
    // 使用unwrap的原因在于,常规情况下我们知道这个调用Regex::new的返回不会出现异常
    let re = Regex::new(r"(\w{5})").unwrap();
    let text = "dcode";


    // println!("Found match? {}", re.is_match(text));
    match re.captures(text) {
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()),
        None => println!("there is not match")
    } 
    
}
