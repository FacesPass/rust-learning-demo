const rust_test = require('./index.node')

let testText = 'Nice to meet you'

console.log(rust_test.count_words(testText, "you"))
