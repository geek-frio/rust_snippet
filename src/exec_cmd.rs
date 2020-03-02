use std::process::Command;

pub fn run(){
    // python dcode.py
    let mut cmd = Command::new("python");
    cmd.arg("dcode.py");

    match cmd.output() {
        Ok(o) => {
            //do stuff
            unsafe{
                String::from_utf8_unchecked(o.stdout);
                println!();
            }
        },
        Err(e) => {
            println!("there is an err:{}", e);
        }
    }

}