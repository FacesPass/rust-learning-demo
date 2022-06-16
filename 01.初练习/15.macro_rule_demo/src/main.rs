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

macro_rules! find_min {
    ($x: expr) => {
        $x
    };
    ($x: expr, $($y: expr), +) => {
        std::cmp::min($x, find_min!($($y), +))
    };
}

macro_rules! calculate {
    (eval $e: expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}",stringify!($e), val);
        }
    }};

    (eval $e: expr, $(eval $x: expr), +) => {{
        calculate! {eval $e}
        calculate! {$(eval $x), +}
    }};
}

fn main() {
    // println!("请输入第一个值:");
    // let mut input = String::new();
    // scanline!(input);
    // println!("Hi: {}", input);
    // println!("请输入第二个值:");
    // let a = scanline!();
    // println!("Hi: {}", a);

    // let min = find_min!(1, 2, 3);
    // println!("{}", &min);

    calculate! {
        eval 1 + 1,
        eval 3 + 4,
        eval (2 * 3) + 1
    };
}
