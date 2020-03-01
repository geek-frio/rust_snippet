struct Person {
    name: String,
    age: u8
}

trait HasVoiceBox {
    //Speak
    fn speak(&self);
    //Check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self){

    }

    fn can_speak(&self) -> bool {
        false
    }
}

pub fn run(){
    let person  = Person {
        name: String::from("Bob"),
        age: 21
    };
    person.speak();
    person.can_speak();
}