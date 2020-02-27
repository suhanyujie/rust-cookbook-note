mod algorithm;
mod notes;

use std::process;
use algorithm::*;
use notes::copy_trait;

fn main() {
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
