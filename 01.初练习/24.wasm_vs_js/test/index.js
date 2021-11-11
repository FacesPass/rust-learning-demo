// const n = 35

// console.log(`斐波那契对照变量 n = ${n}`)

// JS 版本的斐波那契数列
// function fib_js(n) {
//   if (n < 2) {
//     return 1
//   } else {
//     return fib_js(n - 1) + fib_js(n - 2)
//   }
// }

// const start = performance.now()
// fib_js(n)
// const end = performance.now()

// console.log('JS 斐波那契数列:', `${(end - start).toFixed(2)} ms`)



// 异步引入 wasm 相关文件
// import('./wasm/wasm_vs_js').then(test => {
//   const start = performance.now()
//   test.fib(n)
//   const end = performance.now()

//   console.log('wasm 斐波那契数列:', `${(end - start).toFixed(2)} ms`)
// })

function bubble_sort(nums) {
  for (let i = 0; i < nums.length; i++) {
    for (let j = 0; j < nums.length - 1; j++) {
      if (nums[j] > nums[j + 1]) {
        let temp = nums[j]
        nums[j] = nums[j + 1]
        nums[j + 1] = temp
      }
    }
  }

  return nums
}


const n = 3000;
const testArr = []
for (let i = 0; i < n; i++) {
  const randomNum = Math.floor(Math.random() * n)
  testArr.push(randomNum)
}

// const start = performance.now()
// bubble_sort(testArr)
// const end = performance.now()
// console.log('JS 冒泡排序:', `${(end - start).toFixed(2)} ms`)

import('./wasm/wasm_vs_js').then(test => {
  const start = performance.now()
  test.bubble_sort(testArr)
  const end = performance.now()

  console.log('wasm 冒泡排序:', `${(end - start).toFixed(2)} ms`)
})

