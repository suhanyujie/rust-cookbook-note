extern crate lazy_static;

use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;
use lazy_static::*;

lazy_static! {
    static ref SUM: MyData = MyData::new(0);
    // 全局变量不能直接修改它
    static ref SUM0: isize = 0;
}

#[derive(Debug)]
struct MyData0(Rc<isize>);

impl MyData0 {
    pub fn new(num: isize) -> MyData0 {
        MyData0(Rc::new(num))
    }
}

#[derive(Debug)]
pub struct MyData(Arc<Mutex<isize>>);

impl MyData {
    pub fn new(num: isize) -> MyData {
        MyData(Arc::new(Mutex::new(num)))
    }
}

const COUNT: isize = 100;
// static mut SUM: isize = 0;

// 测试数据竞争
// 事实上，用普通的方法很难构造出存在数据竞争的场景，这就是 Rust 强大的安全性特性所带来的好处
pub fn test_is_competition() {
    let num = *SUM0;
    println!("the num is:{:?}", num);
    let mut data1: isize = num;
    let handle1 = thread::spawn(move || {
        for i in 0..COUNT {
            data1 += 1;
            *SUM0 = data1;
            println!("thread1 the result is: {:?}", data1);
        }
    });
    let mut data2: isize = 0;
    let handle2 = thread::spawn(move || {
        for i in 0..COUNT {
            data2 -= 1;
            *SUM0 = data2;
            println!("thread2 the result is: {:?}", data2);
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 解决线程间的数据竞争
pub fn test_resolve_competition() {
    match &*SUM {
        MyData(data1) => {
            let data2 = Arc::clone(&data1);
            let handle1 = thread::spawn(move || {
                for i in 0..COUNT {
                    let mut tmp1 = data2.lock().unwrap();
                    *tmp1 += 1;
                    println!("thread1 the result is: {:?}", tmp1);
                }
            });
            let data3 = Arc::clone(&data1);
            let handle2 = thread::spawn(move || {
                for i in 0..COUNT {
                    let mut tmp1 = data3.lock().unwrap();
                    *tmp1 -= 1;
                    println!("thread2 the result is: {:?}", tmp1);
                }
            });
            handle1.join().unwrap();
            handle2.join().unwrap();
        }
        _ => {
            panic!("get sum data error");
        }
    }
}
