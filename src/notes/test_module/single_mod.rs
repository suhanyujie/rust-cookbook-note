pub mod single_mod {
    pub mod my_mod1 {
        pub fn mod1_func1() {
            println!("file:single_mod-single_mod-my_mod1-mod1_func1");
        }
    }

    pub fn func_0() {
        // 调用父级模块下的函数
        println!("调用父级模块下的函数:");
        super::level1_mod1::mod1_func1();
        // 调用同级下的模块下的函数
        println!("调用同级下的模块下的函数:");
        my_mod1::mod1_func1();
        println!("file:single_mod-single_mod-func_0");
    }
}

pub mod level1_mod1 {
    pub fn mod1_func1() {
        println!("file:single_mod-level1_mod1-mod1_func1");
    }
}
