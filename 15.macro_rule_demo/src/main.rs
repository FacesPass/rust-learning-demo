use std::io::stdin;

// 可以按行读取输入值的宏
// 两种模式，一种是需要传入参数，一种不需要
macro_rules! scanline {
    ($x:expr) => {
        stdin().read_line(&mut $x).unwrap();
    };
    () => {{
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    }};
}

fn main() {
    println!("请输入第一个值:");
    let mut input = String::new();
    scanline!(input);
    println!("Hi: {}", input);
    println!("请输入第二个值:");
    let a = scanline!();
    println!("Hi: {}", a);
}
