// use 声明向当前作用域引入了两个特型（trait）：Write (write!宏) 和 FromStr (u64::from_str)
use std::io::Write;
use std::str::FromStr;

fn get_largest(v: &Vec<u64>) -> u64 {
    let mut max = &v[0];
    for i in v {
        if i > max {
            max = i;
        }
    }

    *max
}

fn main() {
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(u64::from_str(&arg).unwrap());
    }

    if args.len() == 0 {
        writeln!(std::io::stderr(), "args without number value").unwrap();

        std::process::exit(1);
    }

    let max = get_largest(&args);
    println!("max number is {}", max);
}
