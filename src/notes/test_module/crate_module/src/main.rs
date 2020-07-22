mod services;

use services::user_service;

fn main() {
    println!("hello main");
    user_service::get_username();
}
