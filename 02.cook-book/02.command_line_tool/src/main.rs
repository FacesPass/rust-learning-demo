#[macro_use]
extern crate colour;
use clap::{App, Arg};

fn main() {
    let matchs = App::new("测试程序")
        .version("0.0.1")
        .author("王大锤")
        .about("一个测试命令行输入的工具")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("输入文件"),
        )
        .arg(
            Arg::with_name("num")
                .short("n")
                .long("number")
                .takes_value(true)
                .help("输入一个比你最喜欢的数字小5的数"),
        )
        .get_matches();

    let file = matchs.value_of("file").unwrap_or("input.txt");
    blue_ln!("你输入的文件是：{:?}", file);

    let num_str = matchs.value_of("num");
    match num_str {
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("你最喜欢的数字是：{:?}", n + 5),
            Err(_) => red_ln!("你传入的不是一个数字啊"),
        },
        None => red_ln!("我不知道你喜欢的数字是什么，请重新按要求输入"),
    }
}

// cargo run -- -f aaa.txt -n 1
