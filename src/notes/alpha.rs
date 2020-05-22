#![feature(dyn_trait)]
/// dyn trait
use std::mem;

trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn fly(&self) {
        println!("duck fly...");
    }
}

impl Bird for Swan {
    fn fly(&self) {
        println!("swan fly...");
    }
}

fn print_traitobject(p_duck: &dyn Bird) {
    // let p1 = p_duck::fly as usize;
    // println!("{:?}", p_duck);
}

pub fn run() {
    let d1 = Duck {};
    let d2 = &d1;
    let d3 = d2 as &dyn Bird;
    print_traitobject(d2);
}
