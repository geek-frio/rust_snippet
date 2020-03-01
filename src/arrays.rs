use std::mem;

//Length is fixed
pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);
    // Get single value
    println!("Single Value:{}", numbers[0]);
    numbers[2] = 20;
    numbers[2] = 40;
    // stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    // Create slices
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice1: &[i32] = &numbers[0..2];

    println!("Slice1:{:?}", slice1);
}
