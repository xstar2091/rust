// 枚举是类型，大写字母开头
enum IpAddrKind {
    // 枚举值大写字母开头
    V4,
    V6
}

// 枚举用作函数参数，用统一类型处理不同的数据类型
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => { println!("route: ip_addr_kind_1 is V4"); },
        IpAddrKind::V6 => { println!("route: ip_addr_kind_1 is V6"); },
    }
}

// 其他语言需要将类型和数据组合为结构体或者类，才能表达特定数据
// rust可以直接将数据附加到每一个变体
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 除此之外，还可以将IpV4和IpV6分别包装为结构体，作为枚举的变体数据
struct IpV4Addr {
    // 略
}
struct IpV6Addr {
    // 略
}
enum IpAddr2 {
    V4(IpV4Addr),
    V6(IpV6Addr)
}

// 枚举可以内嵌各式各样的数据
// rust可以避免为每种类型创建结构体 + 枚举
enum Message {
    Quit,                       // 纯标签，状态位
    Move { x: i32, y: i32 },    // 结构体变体，数据有语义，需要命名字段，顺序不固定
    Write(String),
    ChangeColor(i32, i32, i32), // 元组结构体变体，数据顺序固定，不关心字段名
}

// 枚举可以定义方法
impl Message {
    // 第一个参数是&self，所以match self其实匹配的是只读借用，所有权还在
    // 对比一下，如果第一个参数是self，所有权就被转移走了，调用一次call后，self不可再用
    fn call(&self) {
        match self {
            Message::Quit => println!("call: Message::Quit"),
            Message::Move { x, y } => { println!("call: Message::Move: {},{}", x, y); },
            Message::Write(msg) => { println!("call: Message::Write: {}", msg); }
            Message::ChangeColor(r, g, b) => println!("call: Message::ChangeColor: {},{},{}", r, g, b),
        }
    }

    // default语义：匿名变量
    fn call_move(&self) {
        match self {
            Message::Move { x, y } => { println!("call_move: Message::Move: {},{}", x, y); }
            _ => println!("call_move: default")
        }
    }

    // default语义：通配绑定变量
    // other是通配绑定变量，会匹配所有未被覆盖的分支
    // other就是枚举值本身
    // 写成other时，一般会在后面的代码中使用；如果后面不需要使用，建议写成_
    fn call_move_2(&self) {
        match self {
            Message::Move { x, y } => { println!("call_move_2: Message::Move: {},{}", x, y); }
            other => println!("call_move_2: default")
        }
    }

    fn call_move_range_1(&self) {
        match self {
            Message::Move { x, y } if *x >= 0 && *x <= 9 && *y >= 10 && *y <= 20 => {
                println!("x in [0,9], y in [10,20]: {},{}", x, y);
            },
            _ => println!("call_move_range: default")
        }
    }
    fn call_move_range_2(&self) {
        match self {
            Message::Move { x, y }
                if (0..=9).contains(x) && (10..=20).contains(y) => {
                println!("x in [0,9], y in [10,20]: {},{}", x, y);
            },
            _ => println!("call_move_range: default")
        }
    }
    fn call_move_range_3(&self) {
        match self {
            Message::Move { x, y } => match (x, y) {
                (x @ 0..=9, y @ 10..=20) => {
                    println!("{},{}", x, y);
                }
                _ => println!("call_move_range: default"),
            },
            _ => println!("call_move_range: default"),
        }
    }

    // match可以根据不同的变体返回数据
    fn get_index(&self) -> i32 {
        match self {
            Message::Quit => 0,
            Message::Move { .. } => 1,
            Message::Write(_) => 2,
            Message::ChangeColor(_, _, _) => 3,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    // 枚举赋值
    let ip_addr_kind_1 = IpAddrKind::V4;
    let ip_addr_kind_2 = IpAddrKind::V6;
    // 枚举必须匹配所有变体
    // 最典型的是match
    match ip_addr_kind_1 {
        IpAddrKind::V4 => { println!("ip_addr_kind_1 is V4"); },
        IpAddrKind::V6 => { println!("ip_addr_kind_1 is V6"); },
    }
    match ip_addr_kind_2 {
        IpAddrKind::V4 => { println!("ip_addr_kind_2 is V4"); },
        IpAddrKind::V6 => { println!("ip_addr_kind_2 is V6"); },
    }
    route(ip_addr_kind_1);
    route(ip_addr_kind_2);
    let ip_addr_1 = IpAddr::V4(127, 0, 0, 1);
    let ip_addr_2 = IpAddr::V6(String::from("::1"));
    match ip_addr_1 {
        IpAddr::V4(v1, v2, v3, v4) => { println!("ip_addr_1 is V4: {}.{}.{}.{}", v1, v2, v3, v4); },
        IpAddr::V6(v) => { println!("ip_addr_1 is V6: {}", v); },
    }
    match ip_addr_2 {
        IpAddr::V4(v1, v2, v3, v4) => { println!("ip_addr_2 is V4: {}.{}.{}.{}", v1, v2, v3, v4); },
        IpAddr::V6(v) => { println!("ip_addr_2 is V6: {}", v); },
    }
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 1, y: 2 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(0, 128, 255);
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
    msg1.call_move();
    msg2.call_move();
    msg3.call_move();
    msg4.call_move();
    msg1.call_move_2();
    msg2.call_move_2();
    msg3.call_move_2();
    msg4.call_move_2();
    println!("Message::Quit        index: {}", msg1.get_index());
    println!("Message::Move        index: {}", msg2.get_index());
    println!("Message::Write       index: {}", msg3.get_index());
    println!("Message::ChangeColor index: {}", msg4.get_index());
    // 简单控制流，在不写match的情况下，用if从枚举中取值
    println!("if-else");
    if let Message::Move{x, y} = msg1 {
        println!("test if msg1, Message::Move: x: {}, y: {}", x, y);
    } else {
        println!("test if msg1");
    }
    if let Message::Move{x, y} = msg2 {
        println!("test if msg2, Message::Move: x: {}, y: {}", x, y);
    } else {
        println!("test if msg2");
    }

    println!("if-else if");
    if let Message::Move{x, y} = msg1 {
        println!("test if msg1, Message::Move: x: {}, y: {}", x, y);
    } else if let Message::Move{x, y} = msg2 {
        println!("test if msg2, Message::Move: x: {}, y: {}", x, y);
    } else if let Message::Move{x, y} = msg3 {
        println!("test if msg3, Message::Move: x: {}, y: {}", x, y);
    } else if let Message::Move{x, y} = msg4 {
        println!("test if msg4, Message::Move: x: {}, y: {}", x, y);
    }
}