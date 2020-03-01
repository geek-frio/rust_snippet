pub fn run(){
    println!("Hello, World!");
    println!("Number:{}", 1);
    println!("Number:{}, Name:{}", 1, "abc");

    println!("{0}, {1}, {2}", "a", "b", "c");
    println!("{name}", name="adfdas");
    println!("Binary: {:b} Hex:{:x} Octal:{:o}", 10, 10, 10 );
    println!("{:?}", (12, true));
}