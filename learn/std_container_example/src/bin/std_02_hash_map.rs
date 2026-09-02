use std::collections::HashMap;

fn create() {
    println!("-----------------create-----------------");
    // new
    let map: HashMap<i32, i32> = HashMap::new();
    // with_capacity
    let map: HashMap<i32, i32> = HashMap::with_capacity(10);
    // from/collect
    let vec = vec![("a", 1), ("b", 2)];
    let map: HashMap<_, _> = vec.into_iter().collect();
    println!("from collector: {:?}", map);
}

fn insert_and_update() {
    println!("-----------------insert_and_update-----------------");

    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("new(insert): {:?}", map);
    // insert，其实是update。
    // 如果key不存在，则插入。如果key已经存在，则更新对应的值
    map.insert("a", 1);
    println!("insert a-1: {:?}", map);
    map.insert("a", 2);
    println!("insert a-2: {:?}", map);
    println!();

    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("new(entry.or_insert): {:?}", map);
    // entry.or_sert: 取值，取到的是可变引用
    // 无论 key 是否存在，都会先创建/计算值，再决定是否插入
    // 如果key存在，则取值。如果key不存在，则插入map，再取值
    let result = map.entry("a").or_insert(1);
    println!("result: {}", result);
    println!("entry(\"a\").or_insert(1): {:?}", map);
    let result = map.entry("a").or_insert(2);
    println!("result: {}", result);
    println!("entry(\"a\").or_insert(2): {:?}", map);
    println!();

    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("new(entry.or_insert_with): {:?}", map);
    // entry.or_sert_with: 取值，取到的是可变引用
    // 只在 key 不存在时才执行闭包来生成值（惰性求值）
    // 如果key存在，则取值。如果key不存在，则插入map，再取值
    let result = map.entry("a").or_insert_with(|| 1);
    println!("result: {}", result);
    println!("entry.or_insert_with: {:?}", map);
    let result = map.entry("a").or_insert_with(|| 1);
    println!("result: {}", result);
    println!("entry.or_insert_with: {:?}", map);
    println!();

    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("new(entry.and_modify.or_insert): {:?}", map);
    // 计数器经典写法
    // 如果key存在，先调用and_modify再调用entry。
    // 如果key不存在，先调用or_insert再调用entry
    let result = map.entry("a").and_modify(|v| *v += 1).or_insert(1);
    println!("result: {}", result);
    println!("entry(\"a\").and_modify.or_insert(1): {:?}", map);
    let result = map.entry("a").and_modify(|v| *v += 1).or_insert(1);
    println!("result: {}", result);
    println!("entry(\"a\").and_modify.or_insert(1): {:?}", map);
    let result = map.entry("a").and_modify(|v| *v += 1).or_insert(1);
    println!("result: {}", result);
    println!("entry(\"a\").and_modify.or_insert(1): {:?}", map);
}

fn main() {
    create();
    insert_and_update();
}