use std::convert::AsRef;

fn main() {
    // demo0_1();
    demo0_3();

    // demo1();
    // demo1_1();
    // demo1_2();
}

fn demo1() {
    let s1 = String::from("hello");
    // bad case
    // let s2: &String = s1.as_ref();
    // good case
    let s2 = &s1;
    println!("{:?}", s2);
}

struct MyStr(String);

impl AsRef<str> for MyStr {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

fn demo1_1() {
    let s1 = MyStr(String::from("hello"));
    let s2 = s1.as_ref();
    println!("{:?}", s2);
}

fn demo1_2() {
    let s1 = "hello world";
    let s2: &str = s1.as_ref();
    println!("{:?}", s2);
}


fn demo0() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
}

fn demo0_1() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name {
        Some(ref name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
}

fn demo0_2() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match option_name.as_ref() {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
}

fn demo0_3() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match &option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name);
}

/*
# Rust's as_ref vs as_deref
>* 作者：[suhanyujie](https://github.com/suhanyujie)
>* 来源：https://github.com/suhanyujie/rust-cookbook-note
>* 深度参考：https://www.fpcomplete.com/blog/rust-asref-asderef/
>* tags：Rust，as_ref，deref
>* tips：如有不当之处，还请指正~

今日在网上看到一段[代码](https://www.fpcomplete.com/blog/rust-asref-asderef/)：

```rust
// can't compile
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

原因是 String 类型没有实现 AsRef Trait，因而不能调用 as_ref() 方法。不过发现编译器为 `OsStr`、`Path`、`[u8]`、`str` 类型实现了 AsRef Trait。因此，同样的代码发生在 str 上，则可以通过编译：

```rust
fn demo1_2() {
    let s1 = "hello world";
    let s2: &str = s1.as_ref();
    println!("{:?}", s2); // hello world
}
```

与此同时，如果你声明了一个类型，并为其实现了 AsRef，则也能调用 as_ref() 方法。

```rust
struct MyStr(String);

impl AsRef<str> for MyStr {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

fn demo1_1() {
    let s1 = MyStr(String::from("hello"));
    let s2 = s1.as_ref();
    println!("{:?}", s2);
}
```

回到文章一开始提到的例子，为了能让其通过编译，我们需要调整一下，在 match 时，增加引用符号 —— `&`：

```
fn main() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match &option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
    println!("{:?}", option_name);
}
```

上面这段代码在 RFC 2005 "match ergonomics" landed in 2016 实现之前，是无法编译的。而需要这样写才能通过编译：

```rust
fn main() {
    let option_name: Option<String> = Some("Alice".to_owned());
    match &option_name {
        &Some(ref name) => println!("Name is {}", name),
        &None => println!("No name provided"),
    }
    println!("{:?}", option_name);
}
```

这样，类型就非常明确了，我们 match 的是 `&Option<String>` 类型，在 `&Some` 分支中，要表示借用，我们需要引入 `ref` 关键字。
重要的是，这些繁琐的东西不再需要了。RFC 2005 的实现，使 match Option 使用起来更加便捷。



## 参考
* Rust's as_ref vs as_deref https://www.fpcomplete.com/blog/rust-asref-asderef/
* Using Deref and AsRef for function arguments https://riptutorial.com/rust/example/16352/using-deref-and-asref-for-function-arguments
* Rust 类型转换 https://www.cnblogs.com/ywxt/p/11801778.html
* AsRef、Deref和Borrow有哪些区别？ https://www.zhihu.com/question/470049587











*/
