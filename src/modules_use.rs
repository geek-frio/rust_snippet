mod eggs {
    pub fn have_egg() {}

    // module套module的情况
    pub mod eggegg {
        pub fn have_egg(){

        }
    }
}

pub fn run() {
    eggs::eggegg::have_egg();
}
