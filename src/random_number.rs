extern crate rand;
use rand::Rng;

// 依赖里面假如rand的依赖
pub fn run() {
    let random_number = rand::thread_rng().gen_range(1, 10);
    println!("random_number:{}", random_number);
}
