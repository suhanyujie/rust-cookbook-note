mod algorithm;
mod notes;
mod command;

use algorithm::*;
use notes::copy_trait;
use std::process;

fn main() {
    // 命令行测试
    command::try_with_test_param();
    process::exit(-1);

    copy_trait::sort_for_struct();
    copy_trait::test_func();
    copy_trait::no_copy();
    copy_trait::has_copy();
    process::exit(-1);

    test();
    test_point();
    for i in 0..1000000 {
        let n1 = mt_rand(0, 100);
        if n1 == 0 {
            println!("Rand num is:{}, times is: {}", n1, i);
            break;
        }
        println!("Rand num is:{}", n1);
    }
}
