extern crate rand;

use rand::Rng;
use rand::distributions::{Distribution, Uniform, Standard, Alphanumeric};

/// 测试函数
pub fn test() {
    let a1 = if true {
        1
    } else {
        2
    };
    println!("the num is:{}", a1);
}

/// 生成一个 `u8` 类型的随机数
/// 调用示例 `let n1 = gen_a_u8_num(0, 100);`
pub fn gen_a_u8_num() -> u8 {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    return n1;
}

/// 生成一个 i32 的随机数
/// 调用示例 `let n1 = gen_a_i32_num(0, 100);`
pub fn gen_a_i32_num() -> i32 {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen();
    return num;
}

/// 生成指定范围内的随机数
/// 调用示例 `let n1 = gen_random_within_range(0, 100);`
pub fn gen_random_within_range(min: isize, max: isize) -> isize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(min, max);
    return num as isize;
}

/// 用更好的方式生成指定范围内的随机数
/// 调用示例 `let n1 = mt_rand(0, 100);`
pub fn mt_rand(min: isize, max: isize) ->isize {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(min..max);
    let num = die.sample(&mut rng);
    num as isize
}

// 声明一个类型，获取其子成员值是随机的该类型值
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point{
            x: rand_x,
            y: rand_y,
        }
    }
}

pub fn test_point() {
    let mut rng = rand::thread_rng();
    let rand_point: Point = rng.gen();
    println!("random point is: {:#?}", rand_point);
}

/// 从ASCII字符串中获取随机密码
pub fn get_random_string(length: u8) ->String {
    let mut rng = rand::thread_rng();
    let rand_str: String = rng.sample_iter(&Alphanumeric)
                .take(length as usize)
                .collect();

    return rand_str;
}

#[test]
fn test_get_random_string() {
    let len: u8 = 3;
    let s1 = get_random_string(len);
    println!("random string is:{}", s1);
    assert_eq!(s1.len(), len as usize);
}

/// 从给定字符串中获取随机字符串
pub fn get_random_string_from_customer_str(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let customer_str: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";
    let rand_str: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0, customer_str.len());
                customer_str[idx] as char
            })
            .collect();
    rand_str
}

#[test]
fn test_get_random_string_from_customer_str() {
    let len: u8 = 12;
    let s1 = get_random_string_from_customer_str(len);
    println!("random string is:{}", s1);
    assert_eq!(s1.len(), 1);
}

