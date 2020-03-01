pub fn run() {
    let mut x = 10;
    // 在离开下面这个scope以后,x的值恢复原样
    {
        let x = 15;
    }
    // 同时也能更改的x的类型
    let x = "X is a string";
    println!("x is {}", x);
    let x = true;
    println!("x is {}", x);
}
