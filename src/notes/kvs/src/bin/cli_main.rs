use clap::{App, Arg, SubCommand};
use env_logger;
use log::{debug, error, info, log_enabled, Level};

/// 命令行方式使用 kvs
fn main() {
    some_init();
    info!("[kvs] start...");
    match cli_app() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}

fn some_init() {
    env_logger::init();
}

/// 错误类型
#[derive(Debug)]
enum MyErr {
    Reason(String),
}

/// 命令行应用
/// 实现命令行下 KV 数据库的交互，支持 set/get/rm 操作
fn cli_app() -> Result<(), MyErr> {
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
                        .index(1)
                        .short("k")
                        .required(true)
                        .help("Type the key"),
                )
                .arg(
                    Arg::with_name("value")
                        .index(2)
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
        ("set", Some(set_cmd)) => {
            let key = set_cmd.value_of("key");
            let value = set_cmd.value_of("key");
            if key.is_none() || value.is_none() {
                return Err(MyErr::Reason("请输入合适的 set 的键和值。".into()));
            }
            dbg!(set_cmd.value_of("key"));
            dbg!(set_cmd.value_of("value"));
            // println!("{:?}", ext_args);
        }
        ("get", Some(_get_cmd)) => {}
        ("rm", Some(_rm_cmd)) => {}
        _ => {}
    }
    Ok(())
}
