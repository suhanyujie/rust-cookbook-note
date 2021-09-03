use std::convert::AsRef;

fn main() {
    demo1();
}

fn demo1() {
    // let s1 = String::from("hello");
    // let s2: &String = s1.as_ref();
    // println!("{:?}", s2);
}

impl AsRef<String> for String {
    // fn as_ref(&self) -> &T {
    //     todo!()
    // }
}

fn demo0() {
    // let option_name: Option<String> = Some("Alice".to_owned());
    // match option_name {
    //     Some(name) => println!("Name is {}", name),
    //     None => println!("No name provided"),
    // }
    // println!("{:?}", option_name);
}

/*
# Rust's as_ref vs as_deref
>* 作者：[suhanyujie](https://github.com/suhanyujie)
>* 来源：https://github.com/suhanyujie/rust-cookbook-note
>* tags：Rust，as_ref，deref
>* tips：如有不当之处，还请指正~

今日在网上看到一段[代码](https://www.fpcomplete.com/blog/rust-asref-asderef/)：

```rust
fn main() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name);
}
```

就是这样一段看似简单的代码，其实还是有点东西的。文章背后的主题是 Rust 的 as_ref 和 as_deref。
我清楚地记得在用 Rust 刷 leetcode 的[题目](https://github.com/suhanyujie/leetcode-rust/blob/master/src/_24_swap-nodes-in-pairs/src/lib.rs)时，经常会使用到相关的函数。
但每次使用时，却从没深入探究过具体的原理，今天看到那段代码，打算具体看看 as_ref 和 as_deref。

先看一下标准库的[相关文档](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

```rust
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}
```

文档描述的是廉价的引用到引用的转换。除此之外，AsRef trait 还会对内部的类型自动解引用。
只看文字可能不容易理解，我们通过一些示例看，就以最常用的字符串类型（String）为例:

```rust
let s1 = String::from("hello");
let s2: &String = s1.as_ref();
println!("{:?}", s2);
```

执行代码发现编译器报错了：

```
error[E0277]: the trait bound `String: AsRef<String>` is not satisfied
 --> src\main.rs:7:26
  |
7 |     let s2: &String = s1.as_ref();
  |                          ^^^^^^ the trait `AsRef<String>` is not implemented for `String`
  |
  = help: the following implementations were found:
            <String as AsRef<OsStr>>
            <String as AsRef<Path>>
            <String as AsRef<[u8]>>
            <String as AsRef<str>>

error: aborting due to previous error
```






## 参考
* Rust's as_ref vs as_deref https://www.fpcomplete.com/blog/rust-asref-asderef/












*/
