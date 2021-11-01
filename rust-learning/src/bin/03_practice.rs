// 函数
fn _valid_email(_intput: &str) -> bool {
    false
}

// 结构体
// 空结构体
struct _Marker;
// 元组结构体
struct _Color(u8, u8, u8);
// 普通结构体
struct _Person {
    name: String,
    age: u8,
}
// 枚举
enum _Status {
    Ok = 0,
    BadName = 1,
    NotFound = 2,
}

// 标签联合
#[derive(Debug)]
enum Option1 {
    Some(u8),
    _None,
}

// 例子
#[derive(Debug)]
enum Gender {
    _Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug, Clone, Copy)]

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    // 变量
    let _a: i32;
    let mut _x: i32;
    let _name = "QAQ";
    let _pi = 3.1415926;
    let mut _v: Vec<u8> = Vec::new();

    // 常量
    const _X: i32 = 9;
    const _PI: f64 = 3.1415926;
    //静态变量
    static _SX: i32 = 18; 
    static mut _SY: i32 = 18;
    static _V: Vec<u8> = Vec::new();
    // 无法通过编译， 需要使用lazy_static
    // static MAP: HashMap<String, String> = HashMap::new();
    let _test_options = Option1::Some(1);
    // println!("{:?}", test_options);
    let _a = (1, 2, 3);
    // println!("{}", a.1);

    // 循环 loop while for
    let n = 45;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
    let arr = [1, 2, 3];
    println!("{}", arr[0]);

    assert_eq!(arr[..], [1, 2, 3]);
    assert_eq!(arr[0..=2], [1, 2, 3]);

    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Leave((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
    println!("\nProcess event:");

    process_event(&event1);
    process_event(&event2);
    process_event(&event3);

    let op = 99;

    // 模式匹配
    match op {
        1 => {
            println!("hello 1");
        }
        2 => println!("hello2"),
        99 => println!("hello 99"),
        100 => println!("hello 100"),
        _ => println!("default"),
    }
    process_message(&event1);

    // 打印命令行参数
    for arg in std::env::args() {
        println!("{}", arg);
    }
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        if i >= n {
            break;
        }
    }
    println!("{}", b);
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        // println!("next val is {}", b);
    }
    println!("{}", b);
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        // println!("next val is {}", b);
    }
    println!("{}", b);
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

// 简单模式匹配
fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

// Test
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
