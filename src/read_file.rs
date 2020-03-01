use std::fs::File;
use std::io::prelude::*;

pub fn run(){
    let mut file = File::open("/home/frio/workspace/rust_sandbox/src/info.txt").expect("Can't open file!");

    let mut contents = String::new();
    // 这里不用用{},因为在方法结束后,contents已经被销毁了
    file.read_to_string(&mut contents).expect("Oops, can not read file...");
    println!("File Contents:{}", &contents);
}