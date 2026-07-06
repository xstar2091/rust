mod ch_10_template;

use std::fmt::{Debug, Display};

pub trait Summary {
    // 只要trait是pub的，其中的函数就是pub的
    fn summarize(&self) -> String;
    // trait定义的函数可以带默认实现
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
    pub location: String,
}

impl Summary for NewArticle {
    // 实现trait的函数默认继承了其pub属性
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_default(&self) -> String {
        format!("{} ({})", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 要实现一个trait，必须实现trait定义的所有函数，但trait中已经有默认实现的，可以不用实现
impl Summary for Tweet {
    // 实现trait的函数默认继承了其pub属性，但是写上pub可以明确表示这是一个对外公开的函数，增加可读性
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 使用trait作为参数调用函数，notify2的语法糖
pub fn notify1(item: &impl Summary) {
    println!("notify1! {}", item.summarize());
}

// trait约束，notify1的完整形式
pub fn notify2<T: Summary>(item: &T) {
    println!("notify2! {}", item.summarize());
}

// notify1的写法简单时真简单，复杂时真复杂
// 但这种写法可以让item1和item2的原始类型不必相同，例如以下调用是正确的
// notify1_2(&article, &tweet);
pub fn notify1_2(item1: &impl Summary, item2: &impl Summary) {
    println!("notify1_2! {}", item1.summarize());
    println!("notify1_2! {}", item2.summarize());
}

// 使用trait约束简化多个trait参数
// 但这不是notify1_2的简化，因为item1和item2的原始类型必须相同
// 以下调用编译失败，因为item1和item2类型不同
// notify2_2(&article, &tweet);
// 以下调用可以成功
// notify2_2(&article, &article);
pub fn notify2_2<T: Summary>(item1: &T, item2: &T) {
    println!("notify2_2! {}", item1.summarize());
    println!("notify2_2! {}", item2.summarize());
}

// 改进notify2_2可以传入不同的类型
pub fn notify2_3<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("notify2_3! {}", item1.summarize());
    println!("notify2_3! {}", item2.summarize());
}

// 使用+语法指定多个trait约束
pub fn notify3_1(item: &(impl Summary + Display)) {}
pub fn notify3_2<T: Summary + Display>(item: &T) {}

// 使用where从句简化trait约束
pub fn notify4_1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
pub fn notify4_2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// 返回实现了trait的类型
// 这里的trait是静态多态，返回类型是在编译器确定的，即只能返回单一类型
// 如果通过if-else返回不同的原始类型，编译会报错
pub fn create_summary_1() -> impl Summary {
    Tweet {
        username: "Jane".to_string(),
        content: "Jane is a better girl".to_string(),
        reply: false,
        retweet: false,
    }
}
pub fn create_summary_2() -> impl Summary {
    NewArticle {
        headline: "The other article".to_string(),
        author: "lx".to_string(),
        content: "with the best".to_string(),
        location: "BeiJing".to_string(),
    }
}
// 这里的trait是静态多态，返回类型是在编译器确定的，即只能返回单一类型
// 如果通过if-else返回不同的原始类型，编译会报错
pub fn create_summary_3(switch: bool) -> impl Summary {
    // 下面这么用就会报错
    if switch {
        // 这里返回NewArticle
    } else {
        // 这里返回Tweet
    }
    // 为了让示例编译通过，这里返回单一类型
    Tweet {
        username: "".to_string(),
        content: "".to_string(),
        reply: false,
        retweet: false,
    }
}

// 只有实现了指定的trait，才会实现特定方法
// 下面这个示例，为所有实现了Display和PartialOrd两个trait的类型实现cmp_display方法
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let article = NewArticle {
        headline: "A new better article".to_string(),
        author: "john".to_string(),
        content: "aa hahaha".to_string(),
        location: "bei jing".to_string(),
    };
    println!("{}", article.summarize());
    println!("{}", article.summarize_default());
    println!("-----------------");

    let tweet = Tweet {
        username: "lily".to_string(),
        content: "go go go".to_string(),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    println!("{}", tweet.summarize_default());
    println!("-----------------");

    // 使用trait作为参数调用函数
    println!("使用trait作为参数调用函数");
    notify1(&article);
    notify1(&tweet);
    println!("-----------------");

    // trait约束
    println!("trait约束");
    notify2(&article);
    notify2(&tweet);
    println!("-----------------");

    // notify1的写法简单时真简单，复杂时真复杂
    println!("notify1的写法简单时真简单，复杂时真复杂");
    notify1_2(&article, &tweet);
    println!("-----------------");

    // 使用trait约束简化多个trait参数
    println!("使用trait约束简化多个trait参数");
    notify2_2(&article, &article);
    println!("-----------------");

    // 改进notify2_2可以传入不同的类型
    println!("改进notify2_2可以传入不同的类型");
    notify2_3(&article, &tweet);
    println!("-----------------");

    // 返回实现了trait的类型
    println!("返回实现了trait的类型");
    let summary1 = create_summary_1();
    let summary2 = create_summary_2();
    println!("summary 1: {}", summary1.summarize());
    println!("summary 2: {}", summary2.summarize());
    println!("-----------------");

    // 只有实现了指定的trait，才会实现特定方法
    println!("只有实现了指定的trait，才会实现特定方法");
    let pair1 = Pair {
        x: 1,
        y: 2,
    };
    let pair2 = Pair {
        x: 1,
        y: 3,
    };
    pair1.cmd_display();
    pair2.cmd_display();
}
