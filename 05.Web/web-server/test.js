const str1 = "Hello world";
const str2 = "你好，世界";

// 返回指定下标的字符
str1.charAt(0); // H
str1.charAt(1); // e
str2.charAt(0); // 你
str2.charAt(1); // 好

// 返回指定下字符的 Unicode 编码
str1.charCodeAt(0); // 72
str2.charCodeAt(0); // 20320

// 拼接字符串
str1.concat(" 大锤"); // Hello world 大锤
str2.concat("，我来啦"); // 你好，世界，我来啦

// 判断字符串是否以某个子字符串开头
str1.startsWith("H"); // true
str2.startsWith("你好"); // true

// 第二个参数也可以指定开始字符的下标位置
str1.startsWith("e", 1);

// 判断字符串是否以某个子字符串结尾
str1.endsWith("d"); // true
str2.endsWith("世界"); // true
// 第二个参数也可以指定结束字符的下标位置
str2.endsWith("你好", 2); // true

// 判断字符串里是否包含某段子字符串
str1.includes("world"); // true
str1.includes("world111"); // false

// 获取子字符串的下标，也可以用于判断是否存在某个子字符串
// 如果不存在则返回 -1
str1.indexOf("world111"); // -1
str1.indexOf("w"); // 6, w在下标为 6 的位置

// 返回子字符串在字符串中的最后一个匹配项。
str1.lastIndexOf("w"); // 6
str1.lastIndexOf("o"); // 7, 匹配到了第二个 o

// 将字符串与正则表达式匹配，并返回包含该搜索结果的数组
// 第一个参数是正则表达式
// 匹配所有字符 o，并返回一个数组
str1.match(/o/g); // [ 'o', 'o' ]
// 匹配拿到所有单词
str1.match(/\w+/g); // [ 'Hello', 'world' ]

// matchAll 返回一个匹配后的迭代器，可以在循环中进行消费
for (let res of str1.matchAll(/o/g)) {
  console.log(res);
}
// 匹配到两个 o ，输出结果：
// [ 'o', index: 4, input: 'Hello world', groups: undefined ]
// [ 'o', index: 7, input: 'Hello world', groups: undefined ]

// 在字符串结尾填充字符以达到给定的长度
// 比如 str1 的最终填充长度为 20, 以 h 来填充
str1.padEnd(20, "h"); // Hello worldhhhhhhhhh

// 在字符串开头填充字符以达到给定的长度
str1.padStart(20, "h"); // hhhhhhhhhHello world

// 将字符串重复拼接
str1.repeat(2); // Hello worldHello world

// 检索字符串中指定的子字符串，或检索与正则表达式相匹配的子字符串。
// 如果没有找到任何匹配的子串，则返回 -1
// 不执行全局匹配，它将忽略标志 g
str1.search(/Hello/); // 0

// 返回指定位置范围内的子字符串, 也可以称为复制字符串
// 第一个参数是开始下标, 第二个参数是结束位置下标
str1.slice(0, 5); // Hello
// 直接复制整个字符串
str1.slice(0); // Hello world

// 类似于 slice 方法, 返回指定位置范围内的子字符串
str1.substring(0, 5); // Hello
str1.substring(0); // Hello world

// 将字符串全部转成小写
str1.toLowerCase(); // hello world

// 将字符串全部转成大写
str1.toUpperCase(); // HELLO WORLD

// trim 移除字符串开头和结尾的空格或换行符
" 啊 啊 啊 ".length; // 7
" 啊 啊 啊 ".trim().length; // 5

// 移除字符串开头的空格或换行符
" 啊 啊 啊 ".trimStart().length; // 6
// 移除字符串结尾的空格或换行符
" 啊 啊 啊 ".trimEnd().length; // 6

// 以指定字符来切割字符串, 返回一个数组
// 在空格位置切割字符串, 返回两个单词
str1.split(" "); // [ 'Hello', 'world' ]
str1.split("w"); // [ 'Hello ', 'orld' ]

// 替换字符串
str1.replace("world", "大锤"); // Hello 大锤
// 将正则匹配到的所有的 o 替换为 ~
str1.replace(/o/g, "~"); // Hell~ w~rld
