extern crate clap;

use clap::{App};

pub fn try_with_version() {
    App::new("ver").version("v0.1.0").get_matches();
}
