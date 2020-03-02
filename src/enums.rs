//Enum are types which have definite values
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //Perform action depending on movement
    match m {
        Movement::Up => println!("Avatar moving up",),
        Movement::Down => println!("Avatar moving down",),
        Movement::Left => println!("Avatar moving left",),
        Movement::Right => println!("Avatar moving right",)
    }
}

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday,
    Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday || && Day::Sunday => {
                return false;
            },
            _ => {
                return true;
            }
        }
    }
}

pub fn run(){
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}