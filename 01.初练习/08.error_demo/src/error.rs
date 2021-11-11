use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseErr {
  //文件解析失败的变体
  Malformd,
  //文件为空的变体
  Empty,
}

#[derive(Debug)]
pub struct ReadErr {
  pub child_err: Box<dyn Error>,
}

// Error 特征要求实现的
impl Display for ReadErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "读取 todo 文件失败！")
  }
}

// Error 特征要求实现的
impl Display for ParseErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "todo 文件解析失败")
  }
}

impl Error for ReadErr {
  fn description(&self) -> &str {
    "TodoList 读取失败"
  }

  fn cause(&self) -> Option<&dyn Error> {
    Some(&*self.child_err)
  }
}

impl Error for ParseErr {
  fn description(&self) -> &str {
    "TodoList 解析失败"
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}
