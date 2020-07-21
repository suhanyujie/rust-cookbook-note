// 函数声明
pub fn one_file_func() {
    println!("file:one_file-one_file_func");
}

trait Human {
    fn speak();
}

// 类型声明
#[derive(Debug)]
pub struct Stu {
    id: i32,
    name: String,
    age: u8,
}

impl Human for Stu {
    fn speak() {
        println!("I speak Chinese.")
    }
}

impl Stu {
    pub fn new() -> Stu {
        Stu {
            id: 1,
            name: String::from("张太一"),
            age: 24,
        }
    }
}
