mod services;
mod helper;

use helper::Helper;
use services::user_service;

fn main() {
    println!("hello main");
    user_service::get_username();
    let h1 = Helper::new();
    println!("{:?}", h1);
}
