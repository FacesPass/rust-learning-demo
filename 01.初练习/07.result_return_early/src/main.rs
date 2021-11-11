//希望将错误给抛出给调用方，让调用方自己处理错误
use std::string::FromUtf8Error;

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let res = match String::from_utf8(str) {
        Ok(str) => str.to_uppercase(),
        Err(err) => return Err(err),
    };

    println!("转换成功, {}", res);
    Ok(res)
}

//可以使用?问好操作符来简化上面的操作
fn str_upper_concise(str: Vec<u8>) -> Result<String, FromUtf8Error> {
    let res = String::from_utf8(str).map(|x| x.to_uppercase())?;
    println!("转换成功, {}", res);
    Ok(res)
}

fn main() {
    let invalid_str = str_upper_concise(vec![197, 198]);
    println!("{:?}", invalid_str);
}
