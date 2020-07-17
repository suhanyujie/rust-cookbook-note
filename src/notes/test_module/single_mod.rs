
pub mod single_mod {
    pub mod my_mod1 {
        pub fn mod1_func1() {
            println!("single_mod-my_mod1-mod1_func1");
        }
    }
}

pub mod level1_func1 {
    pub fn mod1_func1() {
        println!("file:single_mod-level1_func1-mod1_func1");
    }
}