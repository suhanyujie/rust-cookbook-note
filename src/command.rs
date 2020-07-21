extern crate ansi_term;
extern crate chrono;
extern crate clap;

use ansi_term::{Colour, Style};
use chrono::prelude::*;
use clap::{App, Arg};
use std::io::Write;
use std::process::{Command, Output};

/// 简单的命令行程序示例
pub fn try_with_test_param() {
    let matches = App::new("ver")
        .version("v0.1.0")
        .author("suhanyujie<suhanyujie@qq.com>")
        .about("just a version output")
        .arg(
            Arg::with_name("string")
                .short("t")
                .long("testParam")
                .takes_value(true)
                .help("enter a random string"),
        )
        .arg(
            Arg::with_name("git_cmd")
                .short("g")
                .long("git")
                .takes_value(true)
                .help("enter a git cmd"),
        )
        .arg(
            Arg::with_name("todate")
                .short("2")
                .long("to_date")
                .takes_value(true)
                .help("enter a timestamp, tanslate it into date"),
        )
        .get_matches();
    if let Some(test_param) = matches.value_of("testParam") {
        println!("the test param is :{:?}", test_param);
    } else if let Some(git_cmd) = matches.value_of("git_cmd") {
        let out = Command::new("git")
            .arg(git_cmd)
            .output()
            .expect("Failed to execute git command");
        let Output {
            status,
            stdout,
            stderr,
        } = out;
        match std::io::stdout().write_all(&stdout) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    } else if let Some(todate) = matches.value_of("todate") {
        let param_rs: i64 = todate.parse().unwrap();
        let mut ts: DateTime<Utc> = Utc::now();
        if param_rs > 0 {
            ts = Utc.timestamp(param_rs, 0);
        }
        let dt = ts.with_timezone(&FixedOffset::east(8 * 3600));
        let datetime_str = dt.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("datetime is: {}", datetime_str);
    } else {
        println!("the test param is not support!");
    }
}

/// ansi 终端显示
// 具体使用，可以查阅 ansi_term 库的文档
pub fn test_ansi_term() {
    // 颜色设定
    println!("red {}", Colour::Red.paint("red"));
    println!("blue {}", Colour::Blue.paint("blue"));
    println!("green {}", Colour::Green.paint("green"));
    // 加粗设定
    let style1 = Style::new();
    println!("bold {}", style1.bold().paint("bold"));
    // 彩色加粗
    let txt1 = Colour::Yellow.bold().paint("some color and bold");
    println!("some color and bold {}", txt1);
    // 下划线
    println!(
        "underline text {}",
        Colour::Black.underline().paint("underline text")
    );
    // 背景色
    let style = Colour::RGB(31, 31, 31).on(Colour::White);
    println!("background text {}", style.paint("background text"));
    // 自定义颜色 1-255
    let style = Colour::Fixed(100);
    println!("diy color text {}", style.paint("diy color text"));
    // 自定义颜色 RGB 方式
    let style = Colour::RGB(100, 200, 200);
    println!(
        "diy color text by rgb {}",
        style.paint("diy color text by rgb")
    );
}
