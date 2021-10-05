<!--

 * @Author: Binqi Ni
 * @Date: 2021-10-03 15:01:50
 * @LastEditTime: 2021-10-04 20:08:52
 * @LastEditors: Binqi Ni
 * @FilePath: /Rust-Programming/rust-learning/src/bin/04_get_hands_dirty_来写个实用的CLI小工具.md
-->

# 04 写个实用的Command line（CLI） 小工具

## 1. 功能分析

要做一个 HTTPie 这样的工具，我们先梳理一下要实现哪些主要功能：

- 首先是做命令行解析，处理子命令和各种参数，验证用户的输入，并且将这些输入转换成我们内部能理解的参数；
- 之后根据解析好的参数，发送一个 HTTP 请求，获得响应；
- 最后用对用户友好的方式输出响应。

流程图:

![img](https://static001.geekbang.org/resource/image/8f/8c/8fa48ae6e8bd3de42cdf67be4ffb298c.jpg?wh=1920x1106)

我们来看要实现这些功能对应需要用到的库：

- 对于命令行解析，Rust 有很多库可以满足这个需求，我们今天使用官方比较推荐的 clap。
- 对于 HTTP 客户端，在上一讲我们已经接触过 reqwest，我们就继续使用它，只不过我们这次尝个鲜，使用它的异步接口。
- 对于格式化输出，为了让输出像 Python 版本的 HTTPie 那样显得生动可读，我们可以引入一个命令终端多彩显示的库，这里我们选择比较简单的 colored。
- 除此之外，我们还需要一些额外的库：用 anyhow 做错误处理、用 jsonxf 格式化 JSON 响应、用 mime 处理 mime 类型，以及引入 tokio 做异步处理。

## 2. CLI处理

```rust
/*
 * @Author: Binqi Ni
 * @Date: 2021-10-03 21:25:58
 * @LastEditTime: 2021-10-04 17:42:16
 * @LastEditors: Binqi Ni
 * @FilePath: /Rust-Programming/rust-learning/src/bin/04_httpie.rs
 */
use clap::{AppSettings, Clap};

// 定义 HTTPie 的 CLI 的主入口， 它包含若干个子命令
// 下面 /// 的注释是文档， clap 会将其作为 CLI 的帮助

/// a naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Binqi Ni <genesis.nbq@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Clap, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其他 HTTP 方法
}

// get 子命令

/// Feed get with an url and we will retrieve the response for you
#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

// post 子命令。 需要输入一个 URL， 和若干个可选的key = value， 用于提供 json body

/// Feed post with an url and optional key = value pairs. We will post the data
/// as JSON,and retrieve the response for you

#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn main() {
    let opts: Opts =Opts::parse(); 
    println!("{:?}", opts);
}

```

​    代码中用到了 clap 提供的宏来让 CLI 的定义变得简单，这个宏能够生成一些额外的代码帮我们处理 CLI 的解析。通过 clap ，我们只需要**先用一个数据结构 T 描述 CLI 都会捕获什么数据，之后通过 T::parse() 就可以解析出各种命令行参数了**。parse() 函数我们并没有定义，它是 #[derive(Clap)] 自动生成的。

​    目前我们定义了两个子命令，在 Rust 中子命令可以通过 enum 定义，每个子命令的参数又由它们各自的数据结构 Get 和 Post 来定义。

我们运行一下：

```sh

❯ cargo build --quiet && target/debug/httpie post httpbin.org/post a=1 b=2
Opts { subcmd: Post(Post { url: "httpbin.org/post", body: ["a=1", "b=2"] }) }

```

默认情况下，cargo build 编译出来的二进制，在项目根目录的 target/debug 下。可以看到，命令行解析成功，达到了我们想要的功能。

## 3. 加入验证

然而，现在我们还没对用户输入做任何检验，如果有这样的输入，URL 就完全解析错误了：

```sh
❯ cargo build --quiet && target/debug/httpie post a=1 b=2
Opts { subcmd: Post(Post { url: "a=1", body: ["b=2"] }) }
```
