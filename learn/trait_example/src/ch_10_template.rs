// // 泛型函数
// fn larget<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 泛型结构体: 两个字段的类型必须相同
struct Point1<T> {
    x: T,
    y: T,
}

// 泛型方法
impl<T> Point1<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 泛型结构体: 两个字段的类型可以不同
struct Point2<T, U> {
    x: T,
    y: U,
}

// 泛型方法
impl<T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    // 结构体和方法可以分别有自己的泛型参数
    fn mix_up<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

// 泛型枚举
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let p1 = Point1 { x: 5, y: 10 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p1.x(): {}", p1.x());
    let p2 = Point2 { x: 5, y: 15.2 };
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);
    println!("p2.x(): {}, p2.y(): {}", p2.x(), p2.y());
    let p3 = p2.mix_up(Point2 { x: 100, y: 105.15});
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("p3.x(): {}, p3.y(): {}", p3.x(), p3.y());
    let mr1: MyResult<_, f32> = MyResult::Ok(5);
    match mr1 {
        MyResult::Ok(v) => println!("mr1: {}", v),
        MyResult::Err(e) => println!("mr1, err: {:?}", e),
    }
    let mr2: MyResult<i32, _> = MyResult::Err(15.2);
    match mr2 {
        MyResult::Ok(v) => println!("mr2: {}", v),
        MyResult::Err(e) => println!("mr2, err: {:?}", e),
    }
}