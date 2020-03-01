// reference的共存情况
pub fn run(){
    let mut x = 10;
    // 当我们需要使用一个update应用更改某个data的值,
    {
        let dom = &mut x;
        *dom += 1;
    }
    // 我们如果想再次借用这个读引用,上面的写引用就必须被销毁掉,否则会发生race condition.
    // 这样就能保证read和write不会同时发生,造成不一致的情况
    // write是排它的
    // read可以和read共存
    println!("x is {}", &x);
}