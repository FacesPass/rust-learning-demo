use serde::{Deserialize, Serialize};

// 要将任何原生数据类型转换为类 JSON 格式，我们只需在类型上添加一个派生注释即可。
#[derive(Debug, Deserialize, Serialize)]
struct Foo {
    a: String,
    b: u64,
}

impl Foo {
    fn new(a: &str, b: u64) -> Self {
        Foo {
            a: a.to_string(),
            b,
        }
    }
}

fn main() {
    let foo_json = serde_json::to_string(&Foo::new("It's that simple", 101)).unwrap();
    println!("{:?}", foo_json);
    let foo_value: Foo = serde_json::from_str(&foo_json).unwrap();
    println!("{:?}", foo_value);
}
