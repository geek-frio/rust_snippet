pub fn run() {
    let mut count = 0;
    // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);
    //     if count == 20 {
    //         break;
    //     }
    // }

    // while loop (FizzBuff)
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fiz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }
    //Inc 
    count += 1;
    // For range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fiz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }
    }
    
}
