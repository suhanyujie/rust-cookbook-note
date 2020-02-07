extern crate rand;

use rand::Rng;

pub fn gen_a_u8_num() -> u8 {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    return n1;
}