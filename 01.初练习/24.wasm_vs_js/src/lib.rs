use js_sys::Array;
// 预导入所有相关模块
use wasm_bindgen::prelude::*;

// 斐波那契数列
#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

// 冒泡排序
#[wasm_bindgen]
pub fn bubble_sort(nums: Vec<usize>) -> Array {
    let mut nums = nums.to_vec();
    for _i in 0..nums.len() {
        for j in 0..nums.len() - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }

    nums.into_iter().map(JsValue::from).collect()
}
