struct Person {
    name: String,
    age: u8,
}

// 为Person struct实现相应的方法
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("my name is {}, I am:{}", self.name, self.age);
    }
}

pub fn run() {
    let name = Person {
        name: String::from("name"),
        age: 28
    };

    //
    println!("{}", name.to_string());
}
