struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}

impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}

impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        match self.s.find(self.delimiter) {
            Some(pos) => {
                let len = self.delimiter.len_utf8();
                // 前缀
                let s = &self.s[..pos + len];
                // 后缀
                let suffix = &self.s[pos + len..];
                // 更改内部字符串引用，指向剩余部分
                *self.s = suffix;
                Some(s.trim())
            }
            None => {
                let s = (*self.s).trim();
                *self.s = "";

                if s.len() == 0 {
                    None
                } else {
                    Some(s)
                }
            }
        }
    }
}

fn main() {
    let mut s = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);
}
