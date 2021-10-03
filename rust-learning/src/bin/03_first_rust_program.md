# 第一个rust程序

## 一. 第一个rust程序 html to markdown 

> 首先通过cargo new scrape_url 生成一个新项目
>
> 1. 生成一个可执行项目 scrape_url, 入口在src/main.rs
> 2. 在Cargo.toml中添加依赖

```rust
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
html2md = "0.2"
```



- Cargo.toml是Rust项目的配置管理文件， 符合[toml](https://toml.io/cn/v1.0.0)语法。
- 两个依赖
  1. reqwest 是一个HTTP客户端， 它的使用方式和Python下的request类似；
  2. html2md顾名思义， 把HTML文本转换成Markdown。

```rust
use std::fs;

fn main() {
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}
```

- 在命令行下， 进入这个项目的目录， 运行cargo run

### 1. 基本概念

- Rust使用名为cargo的工具来管理项目，类似Node.js 的npm， Golang的 go，用来做依赖管理以及开发过程中的任务管理， 比如编译，运行，测试，代码格式化等等。
- Rust的整体语法偏C/C风格。
  - 函数体使用花括号 {} 包裹
  - 表达式之间使用 ; 间隔
  - 访问结构体成员或者变量使用 点 . 运算符
  - 访问命名空间(namespace) 或者 对象的静态函数使用双冒号 :: 运算符。 
  - 如果要简化对命名空间内部的函数或者数据类型的引用， 可以使用use关键字， 比如 use std::fs。
  - 可执行体的入口函数是main()

### 2. 其他特点

- Rust的变量默认是不可变的， 如果要修改变量的值， 需要显示地使用`mut`关键字
- 除了let/ static/ const/ fn等少数语句外， Rust绝大多数代码都是表达式（expression）。 所以if /while/ for/ loop 都会返回一个值， 函数最后一个表达式就是函数的范沪指， 这和函数式编程语言一致
- Rust支持面向接口编程和泛型编程
- Rust有非常丰富的数据类型和强大的标准库
- rust有非常丰富的控制流程， 包括模式匹配（pattern match）

![img](https://static001.geekbang.org/resource/image/54/43/549bd1fd477ba608ac4a3f785cb49043.jpg?wh=1920x1092)

## 二. 基本语法和数据类型

### 变量和函数

`前面说到，Rust 支持类型推导，在编译器能够推导类型的情况下，变量类型一般可以省略，但常量（const）和静态变量（static）必须声明类型。`

定义变量的时候，根据需要，你可以添加 mut 关键字让变量具备可变性。**默认变量不可变**是一个很重要的特性，它符合最小权限原则（Principle of Least Privilege），有助于我们写出健壮且正确的代码。当你使用 mut 却没有修改变量，Rust 编译期会友好地报警，提示你移除不必要的 mut。

- 在 Rust 下，函数是一等公民，可以作为参数或者返回值。

  ```rust
  
  fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
      f(value)
  }
  
  fn square(value: i32) -> i32 {
      value * value
  }
  
  fn cube(value: i32) -> i32 {
      value * value * value
  }
  
  fn main() {
      println!("apply square: {}", apply(2, square));
      println!("apply cube: {}", apply(2, cube));
  }
  ```

  这里 fn(i32) -> i32 是 apply 函数第二个参数的类型，它表明接受一个函数作为参数，这个传入的函数必须是：参数只有一个，且类型为 i32，返回值类型也是 i32。

- Rust 函数参数的类型和返回值的类型都必须显式定义，如果没有返回值可以省略，返回 unit。函数内部如果提前返回，需要用 return 关键字，否则最后一个表达式就是其返回值。如果最后一个表达式后添加了; 分号，隐含其返回值为 unit。

  ```rust
  
  fn pi() -> f64 {
    3.1415926
  }
  
  fn not_pi() {
    3.1415926;
  }
  
  fn main() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
      pi();
    };
    
    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
  }
  ```



## 三. 数据结构

数据结构是程序的核心组成部分，在对复杂的问题进行建模时，我们就要自定义数据结构。Rust 非常强大，可以用 struct 定义结构体，用 enum 定义标签联合体（tagged union），还可以像 Python 一样随手定义元组（tuple）类型。

`定义一个聊天服务的数据结构`

```rust

#[derive(Debug)]
enum Gender {
  Unspecified = 0,
  Female = 1,
  Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
  id: UserId,
  name: String,
  gender: Gender,
}

#[derive(Debug)]
struct Topic {
  id: TopicId,
  name: String,
  owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
  Join((UserId, TopicId)),
  Leave((UserId, TopicId)),
  Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}
```

简单解释一下：

1. Gender：一个枚举类型，在 Rust 下，使用 enum 可以定义类似 C 的枚举类型
2. UserId/TopicId ：struct 的特殊形式，称为元组结构体。它的域都是匿名的，可以用索引访问，适用于简单的结构体。
3. User/Topic：标准的结构体，可以把任何类型组合在结构体里使用。
4. Event：标准的标签联合体，它定义了三种事件：Join、Leave、Message。每种事件都有自己的数据结构。

在定义数据结构的时候，我们一般会加入修饰，为数据结构引入一些额外的行为。在 Rust 里，数据的行为通过 `trait` 来定义，后续我们会详细介绍 trait，你现在可以暂时认为 trait 定义了数据结构可以实现的接口，类似 Java 中的 interface。

一般我们用 impl 关键字为数据结构实现 trait，但 Rust 贴心地提供了派生宏（derive macro），可以大大简化一些标准接口的定义，比如 `#[derive(Debug)]` 为数据结构实现了 Debug trait，提供了 debug 能力，这样可以通过 {:?}，用 println! 打印出来。

在定义 UserId / TopicId 时我们还用到了 Copy / Clone 两个派生宏，Clone 让数据结构可以被复制，而 Copy 则让数据结构可以在参数传递的时候自动按字节拷贝。在下一讲所有权中，我会具体讲什么时候需要 Copy。

![img](https://static001.geekbang.org/resource/image/15/cb/15e5152fe2b72794074cff40041722cb.jpg?wh=1920x1898)

## 四. 控制流程

`顺序执行就是一行行代码往下执行。在执行的过程中，遇到函数，会发生函数调用。函数调用是代码在执行过程中，调用另一个函数，跳入其上下文执行，直到返回。`

`loop` `while` `for`

`break` `continue`

### 1. loop while for

```rust

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        
        println!("next val is {}", b);
        
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
```

**Rust 的 for 循环可以用于任何实现了 `IntoIterator trait` 的数据结构。**

>因而，for 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None。

在 fib_for 函数中，我们还看到 2…n 这样的语法，想必 Python 开发者一眼就能明白这是 Range 操作，2…n 包含 2<= x < n 的所有值。和 Python 一样，在 Rust 中，你也可以省略 Range 的下标或者上标，比如：

```rust
let arr = [1, 2, 3];
assert_eq!(arr[..], [1, 2, 3]);
assert_eq!(arr[0..=1], [1, 2]);
```

![img](https://static001.geekbang.org/resource/image/e3/6c/e3a96ae58a98f46f98b56yya6378b26c.jpg?wh=1920x2144)



### 2. 模式匹配

​    Rust 的模式匹配吸取了函数式编程语言的优点，强大优雅且效率很高。它可以用于 struct / enum 中匹配部分或者全部内容，比如上文中我们设计的数据结构 Event，可以这样匹配

```rust
fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}
```

#### 简单模式匹配

除了使用 match 关键字做模式匹配外，我们还可以用 if let / while let 做简单的匹配，如果上面的代码我们只关心 Event::Message，可以这么写

```rust
fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);   
    }
}
```



## 五. 错误处理

Rust 没有沿用 C++/Java 等诸多前辈使用的异常处理方式，而是借鉴 Haskell，**把错误封装在 Result 类型中，同时提供了 ? 操作符来传播错误，方便开发**。Result 类型是一个泛型数据结构，T 代表成功执行返回的结果类型，E 代表错误类型。

开始的 scrape_url 项目，其实里面很多调用已经使用了 Result 类型，这里我再展示一下代码，不过我们使用了 `unwrap()` 方法，`只关心成功返回的结果，如果出错，整个程序会终止`。

```rust
use std::fs;
fn main() {
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}
```

如果想让错误传播，可以把所有的 unwrap() 换成 ? 操作符，并让 main() 函数返回一个 Result，如下所示：

```rust
use std::fs;
// main 函数现在返回一个 Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
```



## 六. Rust 项目的组织

当 Rust 代码规模越来越大时，我们就无法用单一文件承载代码了，需要多个文件甚至多个目录协同工作，这时我们可以用 `mod` 来组织代码。

具体做法是：在项目的入口文件 lib.rs / main.rs 里，用 mod 来声明要加载的其它代码文件。如果模块内容比较多，可以放在一个目录下，在该目录下放一个 mod.rs 引入该模块的其它文件。这个文件，和 Python 的 __init__.py 有异曲同工之妙。这样处理之后，就可以用 mod + 目录名引入这个模块了，如下图所示：

![img](https://static001.geekbang.org/resource/image/8e/1d/8eff27daa16a2bab514590f0b567341d.jpg?wh=1761x1381)

在 Rust 里，一个项目也被称为一个 crate。crate 可以是可执行项目，也可以是一个库，我们可以用 cargo new -- lib 来创建一个库。当 crate 里的代码改变时，这个 crate 需要被重新编译。在一个 crate 下，除了项目的源代码，单元测试和集成测试的代码也会放在 crate 里。Rust 的单元测试一般放在和被测代码相同的文件中，使用条件编译 #[cfg(test)] 来确保测试代码只在测试环境下编译。以下是一个单元测试的例子：

```rust

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

集成测试一般放在 tests 目录下，和 src 平行。和单元测试不同，集成测试只能测试 crate 下的公开接口，编译时编译成单独的可执行文件。在 crate 下，如果要运行测试用例，可以使用 cargo test。当代码规模继续增长，把所有代码放在一个 crate 里就不是一个好主意了，因为任何代码的修改都会导致这个 crate 重新编译，这样效率不高。我们可以使用 workspace。一个 workspace 可以包含一到多个 crates，当代码发生改变时，只有涉及的 crates 才需要重新编译。当我们要构建一个 workspace 时，需要先在某个目录下生成一个如图所示的 Cargo.toml，包含 workspace 里所有的 crates，然后可以 cargo new 生成对应的 crates：

![img](https://static001.geekbang.org/resource/image/2b/62/2bf542e266197e04ededc5c4a6e6cf62.jpg?wh=1920x1134)

crate 和 workspace 还有一些更高级的用法，在后面遇到的时候会具体讲解。如果你有兴趣，也可以先阅读 [Rust book](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) 第 14 章了解更多的知识。

## 小结

我们简单梳理了 Rust 的基本概念。通过 let/let mut 定义变量、用 fn 定义函数、用 struct / enum 定义复杂的数据结构，也学习了 Rust 的基本的控制流程，了解了模式匹配如何运作，知道如何处理错误。最后考虑到代码规模问题，介绍了如何使用 mod、crate 和 workspace 来组织 Rust 代码。我总结到图中你可以看看。

![img](https://static001.geekbang.org/resource/image/ed/6a/ed405b83640a52e28d3705f62f32b46a.jpg?wh=1920x1132)
