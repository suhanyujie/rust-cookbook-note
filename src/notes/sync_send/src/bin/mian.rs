fn main() {
    // dyn_object_send();
    deref_of_newtype();
    println!("hello world.")
}

fn dyn_object_send() {
    let f1: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("this is dyn fn object. "));
    std::thread::spawn(move || {
        (*f1)();
        drop(f1);
    });
}

// newtype 的 deref
// 通过实现 Deref、DerefMut 可以让新建的类型支持调用被包装类型的方法
struct MyVec (Vec<i8>);

impl std::ops::Deref for MyVec {
    type Target = Vec<i8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for MyVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
   
}

// newtype 实现 Deref、DerefMut trait 后，可以支持调用 Vec<T> 的方法
fn deref_of_newtype() {
    let mut ori_v: Vec<i8> = vec![1,2,2,4];
    // Vec<i8> 类型原生就支持 push 方法。
    ori_v.push(5);
    assert_eq!(ori_v, vec![1,2,2,4,5]);
    let mut v1 = MyVec(vec![1, 2, 2, 3]);
    // 实现 DerefMut 后，MyVec 类型可以调用内部的 Vec<i8> 的 pop 方法
    v1.push(5);
    assert_eq!(v1.0, vec![1, 2, 2, 3, 5]);
}
