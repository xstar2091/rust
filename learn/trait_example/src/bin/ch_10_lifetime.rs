// 生命周期省略规则：规则全部应用后，能够创建带生命周期的完整函数签名，则编译通过，否则编译失败
// 规则1: 每一个参数都有自己的生命周期参数（有几个引用参数就有几个生命周期参数）
// 规则2: 只有一个输入生命周期参数时，其被赋给所有输出生命周期参数
// 规则3: 有多个输入生命周期参数，其中一个是&self或&mut self时，self的生命周期会赋给所有的输出生命周期参数

// 应用生命周期省略规则1和2
fn my_one(own: &str) -> &str {
    own
}

// 强制将x与y的生命周期统一起来
// 返回值的生命周期不能比x短，也不能比y短
// 即返回值的生命周期是x与y生命周期更短的那一个
fn my_two_1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 如果不参与返回值，可以忽略生命周期
fn my_two_2<'a>(x: &'a str, y: & str) -> &'a str {
    x
}

// 生命周期、泛型同时存在
// 顺序必须严格按照以下形式: 生命周期参数、类型参数、const泛型参数
// where子句中的顺序不重要，但是按照以上顺序写可读性更好
fn foo<'a, T, const N: usize>(x: &'a [T; N]) -> &'a T {
    &x[0]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self, other: &str) -> & str {
        self.part
    }

    // 生命周期约束
    // 'a: 'b 'a的生命周期 >= 'b('a是大圈，'b是里面的小圈)
    // T: 'a  类型T中所有引用的值，其生命周期都 ≥ 'a(T在'a范围内始终是完整可用的)
    // &'a T  引用本身带生命周期
    // <'a>   生命周期参数声明
    // 另一种解释
    // 'a: 'b 'a活的更久
    // T: 'a  T在'a不会悬垂
    // &'a T  引用在'a不会悬垂
    // <'a>   生命周期参数声明
    fn longest<'b>(&self, other: &'b str) -> &'b str
    where
        'a: 'b,
    {
        if self.part.len() > other.len() {
            self.part
        } else {
            other
        }
    }
}

fn main() {
    println!("Hello, world!");
}