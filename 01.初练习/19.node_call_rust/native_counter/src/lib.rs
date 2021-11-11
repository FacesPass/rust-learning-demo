// 预导入 neon 所有相关的属性
use neon::prelude::*;

// 编写函数，cx 里面放着一些关于函数的上下文信息，JsResult是一个泛型的返回类型
fn count_words(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // JS函数的第一个参数，类型为 string
    let text = cx.argument::<JsString>(0)?.value(&mut cx);
    // JS函数的第二个参数，类型为 string
    let word = cx.argument::<JsString>(1)?.value(&mut cx);
    // 返回一个数字，计算文本中出现单词的次数，将所有单词转小写，然后再根据空格分割，再过滤算出个数
    Ok(cx.number(
        text.to_lowercase()
            .split(" ")
            .filter(|s| s == &word)
            .count() as f64,
    ))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("count_words", count_words)?;
    Ok(())
}
