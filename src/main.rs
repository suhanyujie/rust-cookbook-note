mod algorithm;

use algorithm::*;

fn main() {
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
