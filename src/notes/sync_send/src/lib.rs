use std::fmt::Debug;

/// 在一个循环中，对一个变量进行读写操作
fn read_and_write() {
    let mut flag = 1;
    let mut s1 = String::new();
    s1 += "start";
    loop {
        if flag > 10 {
            break;
        }
        println!("string is {}", s1);
        s1.push_str(" hello");
        flag += 1;
    }
    println!("{}", s1);
}

#[derive(Debug)]
struct Stu {
    name: String,
    age: u8,
}

trait Say {
    fn say(&self);
}
impl Say for Stu {
    fn say(&self) {
        println!("saying something");
    }
}

/// box 内放置引用数据
fn box_ref() {
    let s1 = Stu {
        name: String::from("张三"),
        age: 19,
    };
    let o1: Box<&dyn Say> = Box::new(&s1);
    dbg!(o1);
}

impl Debug for &dyn Say {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("&dyn Say")
        .field("object", &self.say())
        .finish()
    }
}
impl Stu {
    fn print(&self) {
        println!("{:?}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_and_write() {
        read_and_write();
        assert!(true);
    }

    #[test]
    fn test_box_ref() {
        box_ref();
    }

    #[test]
    fn test_dyn_object_send() {
        let f1: Box<dyn Fn() + Send + 'static> = Box::new(||println!("this is dyn fn object. "));
        std::thread::spawn(move || {
            (*f1)();
        });
        assert!(false);
    }
}
