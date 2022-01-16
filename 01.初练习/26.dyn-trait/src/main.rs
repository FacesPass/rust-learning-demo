pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct MarkdownFormater;
impl Formatter for MarkdownFormater {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Markdown formatter");
        true
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Rust formatter");
        true
    }
}

pub fn format(input: &mut String, formatters: Vec<Box<dyn Formatter>>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

fn main() {
    let mut text = "hello world".to_string();
    let rust: Box<dyn Formatter> = Box::new(RustFormatter);
    let mark: Box<dyn Formatter> = Box::new(MarkdownFormater);
    let formatters = vec![rust, mark];
    format(&mut text, formatters);
    println!("text: {}", text);
}
