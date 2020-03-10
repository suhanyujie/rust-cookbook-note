extern crate clap;

use clap::{App, Arg};

pub fn try_with_test_param() {
    let matches = App::new("ver").version("v0.1.0")
            .author("suhanyujie<suhanyujie@qq.com>")
            .about("just a version output")
            .arg(Arg::with_name("testParam")
                    .short("t")
                    .long("testParam")
                    .takes_value(true)
                    .help("enter a random string")
            )
            .get_matches();
    if let Some(test_param) = matches.value_of("testParam") {
        println!("the test param is :{:?}", test_param);
    } else {
        println!("the test param is not found");
    }
}
