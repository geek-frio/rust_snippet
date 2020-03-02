pub fn run(){
    /* Replace */
    {
        let my_string = String::from("Rust is fantastic");
        // replace 创建了一个新的String,所以&self不需要是mut的
        println!("After replace:{} ", my_string.replace("fantastic", "great"));
    }
    /* Lines */
    {
        let my_string = String::from("The weathers is \n nice!\noutside mate");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }
    /* Split */
    {
        let my_string = String::from("Leave_asdf_adfa");
        let tokens:Vec<&str> = my_string.split("_").collect();
        for token in tokens {
            println!("[ {} ]", token);
        }
    }
    /*Trim*/
    {
        let my_string = String::from(" My name is xxx   \n\r");
        println!("{}", my_string.trim());
    }
    /*Chars*/
    {
        let my_string = String::from("dcode on Youtube");
        match my_string.chars().nth(4) {
            Some(c) => println!("{}", c),
            None => println!("no character")
        }
    }
}