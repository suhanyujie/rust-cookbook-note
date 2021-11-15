use clap::{App, Arg, SubCommand};
use env_logger;
use log::{debug, error, info, log_enabled, Level};

/// 命令行方式使用 kvs
fn main() {
    some_init();
    println!("hello world...");
    cli_app();
}

fn some_init() {
    env_logger::init();
}

/// 命令行应用
/// 实现命令行下 KV 数据库的交互，支持 set/get/rm 操作
fn cli_app() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .author("SamuelSu<suhanyujie@qq.com>")
        .about("A simple K-V database")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Print the version info of kvs"),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set one value with key")
                .arg(
                    Arg::with_name("key")
                        .short("k")
                        .required(true)
                        .help("Type the key"),
                )
                .arg(
                    Arg::with_name("value")
                        .short("v")
                        .required(true)
                        .help("type the value"),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get one value for key")
                .version("1.3")
                .arg(Arg::with_name("key").short("k").help("Type the key")),
        )
        .subcommand(
            SubCommand::with_name("rm").arg(Arg::with_name("key").short("k").help("Type the key")),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(set)) => {
            let ext_args: Vec<&str> = set.values_of("set").unwrap().collect();
            println!("{:?}", ext_args);
        }
        ("get", Some(get)) => {}
        ("rm", Some(rm)) => {}
        _ => {}
    }
}
