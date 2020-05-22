extern crate clap;

use clap::{App, Arg};
use std::io::Write;
use std::process::{Command, Output};

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
    } else {
        println!("the test param is not support!");
    }
}
