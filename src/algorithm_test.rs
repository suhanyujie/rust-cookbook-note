mod algorithm;

use algorithm::*;

#[test]
fn test_get_random_string() {
    let s1 = get_random_string();
    println!("random string is:{}", s1);
    assert_eq!(1, 2);
}