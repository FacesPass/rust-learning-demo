use regex::Regex;
use std::str::FromStr;

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = reg.captures(s) {
            captures
                .get(0)
                .map_or(Err("Failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "Failed to parse capture string".to_string())
                })
        } else {
            Err("Faild to parse string".to_string())
        }
    }
}

fn parse_should_work() {
    assert_eq!(u32::parse("123abcd"), Ok(123));
    assert_eq!(
        u32::parse("123.45abcd"),
        Err("Failed to parse capture string".into())
    );
    assert_eq!(f64::parse("123.45abcd"), Ok(123.45));
    assert!(f64::parse("abcd").is_err());
}

fn main() {
    parse_should_work();
    println!("result: {:?}", u8::parse("255 hello world"));
}
