## Node 和 Rust 能碰撞出什么火花

### 为什么要让这俩家伙碰撞呢？

**众所周知啊**，有时候我们用 `Node.js` 需要实现一些复杂的计算但是会阻塞主线程，计算时间长了响应会变慢，我们都知道 `Node.js` 适合做 `I/O` 密集型的应用而不适合做 `CPU` 密集型的应用就是这个原因。那怎么去解决这个瓶颈呢？ `Node.js` 是有调用原生模块的能力的，比如调用 `C/C++` 的一些相关模块，但是我们今天的主角并不是 `C/C++` 而是 `Rust` ，`Rust` 也可以通过 `FFI(外部函数接口)` 抽象来创建原生的模块供 `C/C++/Python/Node.js 等等等` 一些语言去进行调用。而在 `Node.js` 中想要调用 `Rust` 模块我们就需要借助 `neon` 的帮助了。

### 了解neon

`neon` ([相关链接](https://github.com/neon-bindings/neon))  ，它包含一组工具和胶水代码，能够帮助 `Node.js` 开发者提高开发效率，允许他们在 `Rust` 中编写原生的 `Node.js` 模块，并在`JavaScript` 代码中无缝集成。使用 neon 可以像在 `C/ C++` 中一样创建一个原生 `Node` 模块，并且使用起来很简单，不会让用户在不安全的代码中 **(Rust 特性之一就是编写安全代码)** 感到害怕或头疼。使用 `Rust` 编写原生模块具有如下的好处：

- 提供原生的性能
- 可以编写多线程的并发程序
- 可以使用 `Rust` 的社区生态，各种开源包
- 可以调用本地操作系统的 `API`

### 应用场景

`Native addon(原生扩展)` 可以做到 `JavaScript` 做不到的一些事情，比如调用系统库、打开一个窗口、调用 `GPU` 等一些系统指令，另外在一些 `CPU` 密集计算的地方，比如说区块链货币计算，文件加密解密等等需要比较高的计算能力，此时我们就可以使用原生模块来进行扩展，原生模块的本质就是一个二进制文件。而前端目前都有哪些比较出名的应用场景呢？

比如：

- swc([相关链接](https://github.com/swc-project/swc))：`swc` 是一个使用 `Rust` 编写的超级超级快的 `Typescript / Javascript`  编译器（类似于 `babel`），它是一个可以同时给 `Rust` 和 `Javascript` 使用的库。想要在 `Rust` 中使用，可以参考这里的 [相关链接](https://rustdoc.swc.rs/swc/) ，想要在 `Javascript` 中使用，可以参考这里的 [相关链接](https://swc.rs/docs/installation/)。
- next.js(> v12.0): 最新的 `next.js` 中也使用到了 `Rust` 构建的原生模块作为编译器， next.js 的编译器是基于上述所说的swc来写的，根据官方的介绍，使用了最新的编译器之后本地重新编译速度提升了 3 倍以上，生产环境的打包速度**提升了 5 倍以上**，并且编译速度**比 Babel 快 17 倍以上**。 [相关链接](https://nextjs.org/blog/next-12#faster-builds-and-fast-refresh-with-rust-compiler) 在这里。
- 另外还有一些第三方开源的使用 `Rust` 编写的 `npm` 包，比如加密算法 [node-rs/bcrypt](https://github.com/napi-rs/node-rs/tree/master/packages/bcrypt) ，中文分词 [node-rs/jieba](https://github.com/napi-rs/node-rs/tree/master/packages/jieba) 等，其中都涉及到复杂的计算。

### 开始了解并实现需求

我们现在希望可以用 `Rust` 来写一 个可以计算一大块文本中给定单词出现次数的函数，然后供给 `Node.js` 侧使用。

在你使用下面的命令生成开发项目文件夹之前，先确保你电脑上已经安装了 `Rust` 和 `Node.js(version > 10.0)` ，然后控制台输入下面命令创建一个开发目录：

```
npm init neon my-project
```

接下来就是按照提示来输入相关的项目信息，全部按回车键跳过就好，后续需要修改可以在 `package.json` 和 `Cargo.toml` 中进行修改。

命令创建完之后，我们可以看见这样的一个目录结构：

```
└─native_counter
    │  .gitignore
    │  Cargo.toml
    │  package.json
    │  README.md
    └─src
       lib.rs
```

在 `src` 目录下，我们可以看到给我们默认生成了 `lib.rs`，我们编写代码的地方就是在 `lib.rs` 中。下面我们看看默认给我们生成的 `Cargo.toml` 文件。

```toml
[package]
# .. 省略这部分

[lib]
crate-type = ["cdylib"]

[dependencies]

[dependencies.neon]
version = "0.9"
default-features = false
features = ["napi-6"]

```

接下来我们看看 `crate-type` 这个字段，它的意思如下：

> `[crate_type = "cdylib"]` - 一个动态的系统库将会产生，类似于C共享库。当编译一个从其它语言加载调用的动态库时这属性将会被使用。在 `Linux` 系统上会生成类型为 `*.so` 的文件，在 `MacOS` 上会生成类型为  `*.dylib` 的文件，在 `Windows` 系统则是 `*.dll` 类型的。

下面的 `[dependencies.neon]` 则是关于 `neon` 依赖的相关信息

### 编写代码

打开文件 `native/src/lib.rs` ，开始编写我们的相关代码：

```rust
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
```

`FunctionContext` 实例作为函数的参数，它包含 JavaScript 中被调用的函数的相关信息，例如参数列表、参数长度、this 绑定，以及其 他一些细节。

编写完成 `Rust` 代码之后，接下来将项目生成 `node` 模块，项目根目录下运行命令：

```
npm install
```

然后依赖会自动安装并且会执行相应的 `cargo` 编译命令，完成之后你可以看到在根目录下生成了一个新文件 `index.node`，这个文件就是我们刚刚编写的 `Rust` 模块代码，在编写我们的测试代码之前我们先了解一下 `Node.js` 加载模块的文件类型

### Node.js 加载文件

`Node.js `里面的 `require` 主要支持三种文件类型：

> **.js**：`.js`文件是我们最常用的文件类型，加载的时候会先运行整个JS文件，然后将前面说的`module.exports`作为`require`的返回值。
>
> **.json**：`.json`文件是一个普通的文本文件，直接用`JSON.parse`将其转化为对象返回就行。
>
> **.node**：`.node`文件是其它语言编译后的二进制文件。

如果没有指定扩展名，比如引入 `const test = require('./index');` ，那么 `Node.js` 会先尝试将其按 `.js` 文件来解析，如果不是 `.js` 文件，再尝试按 `.json` 文件来解析。如果都不是，会尝试按 `.node` 二进制文件解析。

### 进行测试

我们在项目根目录下创建一个 `test.js` 文件用来测试，并编写相关的代码

```js
const rust_test = require('./index.node')

let testText = 'Nice to meet you'

console.log(rust_test.count_words(testText, "you")) // 输出1
```

上面就是写一个简单的 `Node.js` 调用 `Rust` 编写的原生模块的例子了，如果编写更加复杂的例子，也需要有更好的 `Rust` 功底，具体的应用和想象空间就留给你们咯。 

### 相关的其它介绍

自从 `Node.js v10` 发布之后，`Node.js` 推出了用于改善开发原生模块的接口，`N-API`（[相关链接](https://nodejs.org/api/n-api.html)），我们这里面的 neon 构建的原理就是使用了 **N-API**。**Node.js** 对 `native addon` 开发侧暴露的是 **ABI (application binary interface，应用二进制接口)**。

目前社区中大部分的 `Node.js addon` 基本都使用 `C/C++` 开发。`C/C++` 生态非常的繁荣，基本上你想做任何事情都能找到对应的 C/C++ 库。但是 `C/C++` 缺少统一的构建工具链和包管理工具，导致了开发和维护上的很多困难。而 `Rust` 中有现代化的管理工具 `Cargo (很好很强大)`。

但是这么强大的东西，难道它就没有缺点吗？

那是肯定有的，`Native code （C/C++/Rust）`在一些纯计算的场景比纯 `JS` 快非常多，但是一旦使用 `N-API`与 `Node` 的 `JS` 引擎打交道，就会有很大的性能开销(相对计算而言)。在一些简单的计算操作中，使用原生模块进行开发的话，可能速度比纯 `JS` 还要慢，因为在桥接的过程中产生的性能开销已经超过了本身计算的时间，结果得不偿失，所以说为什么推荐在复杂的计算场景之中使用，比如说 `swc` 中的对文件编译打包就是其应用场景之一，这里面的场景就已经足够复杂了。所以我们选择的适合就要看应用场景了。

