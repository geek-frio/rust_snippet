pub fn run(){
    // Immutable string
    let hello = "Hello";
    // Heap string
    let mut hello = String::from("Hello ");
    println!("{}", hello);
    println!("Length({})", hello.len());
    hello.push('W');
    hello.push_str("orld!");
    println!("{}", hello);

    //Get capacity
    println!("Capacity:{} Length:{};", hello.capacity(), hello.len());
    println!("IsEmpty:{}", hello.is_empty());
    // Contains
    println!("Contains:{}", hello.contains("World!"));
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push_str("ch: charsdafsafdsafdas");
    //
    assert_eq!(2, s.len());

}