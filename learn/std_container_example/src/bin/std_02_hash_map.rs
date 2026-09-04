use std::collections::HashMap;

const TITLE_PREFIX_SURFIX_SIZE: usize = 17;
fn print_title(name: &str) {
    let line: String = std::iter::repeat('-').take(TITLE_PREFIX_SURFIX_SIZE).collect();
    println!("{}{}{}", line, name, line);
}

macro_rules! print_func_name {
    () => {
        let bt = std::backtrace::Backtrace::capture();
        let line = bt
            .to_string()
            .lines()
            .next()                     // 第一行
            .unwrap_or("")
            .to_string();
        let func_name = line.split_whitespace()
            .nth(1)
            .unwrap_or("unknown")
            .to_string();
        println!("{}{}{}", "-".repeat(17), func_name, "-".repeat(17));
    };
}

fn create() {
    print_func_name!();
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
    print_func_name!();

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

fn get_from_hash_map() {
    print_func_name!();
    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("new(get): {:?}", map);

    println!("empty map");
    let v = map.get("a");
    println!("get(\"a\"): {:?}", v);
    if let Some(v) = map.get("a") {
        println!("a: {}", v);
    } else {
        println!("a not found");
    }
    println!();

    println!("insert key a");
    map.insert("a", 1);
    let v = map.get("a");
    println!("get(\"a\"): {:?}", v);
    if let Some(v) = map.get("a") {
        println!("a: {}", v);
    } else {
        println!("a not found");
    }
    println!();

    println!("change zhe value");
    if let Some(v) = map.get_mut("a") {
        *v += 1;
    }
    let v = map.get("a");
    println!("get(\"a\"): {:?}", v);
    if let Some(v) = map.get("a") {
        println!("a: {}", v);
    } else {
        println!("a not found");
    }
    println!();

    println!("key contains");
    let exist = map.contains_key("a");
    println!("key contains a: {}", exist);
    let exist = map.contains_key("b");
    println!("key contains b: {}", exist);
    println!();

    println!("is empty");
    let is_empty = map.is_empty();
    println!("is_empty is: {}", is_empty);
    println!();

    println!("map len");
    let len = map.len();
    println!("map len: {}", len);
    println!();
}

fn remove() {
    print_func_name!();
    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();

    println!("map: {:?}", map);
    map.remove("a");
    println!("remove(\"a\"): {:?}", map);
    if let Some((k, v)) = map.remove_entry("b") {
        println!("remove_entry, key:{}, value:{}", k, v);
    }
    println!("remove_entry(\"b\"): {:?}", map);

    map.clear();
    println!("clear: {:?}", map);

    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    for (k, v) in map.drain() {
        println!("drain, key:{}, value:{}", k, v);
    }
    println!("drain(): {:?}", map);
}

fn traverse() {
    print_func_name!();

    // 遍历键值对（不可变）
    println!("遍历键值对（不可变）");
    let map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for (k, v) in &map {
        println!("key:{}, value:{}", k, v);
    }
    println!("after: {:?}", map);
    println!();

    // 遍历键值对（可变）
    println!("遍历键值对（可变）");
    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for (_, v) in &mut map {
        *v += 1;
    }
    println!("after: {:?}", map);
    println!();

    // 遍历键
    println!("遍历键");
    let map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for k in map.keys() {
        println!("key: {}", k);
    }
    println!("after: {:?}", map);
    println!();

    // 遍历键
    println!("遍历值");
    let map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for v in map.values() {
        println!("value: {}", v);
    }
    println!("after: {:?}", map);
    println!();

    // 遍历键
    println!("遍历值（可变）");
    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for v in map.values_mut() {
        *v += 1
    }
    println!("after: {:?}", map);
    println!();

    // 移动遍历，之后map不可再使用
    println!("遍历后map不可再使用");
    let map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("origin: {:?}", map);
    for (k, v) in map {
        println!("key:{}, value:{}", k, v);
    }
    // 从这里开始，map不能再用了
    println!();
}
fn len_and_cap() {
    print_func_name!();
    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2), ("c", 3)].into_iter().collect();
    println!("len: {}, capacity: {}", map.len(), map.capacity());
    map.reserve(10);
    println!("len: {}, capacity: {}", map.len(), map.capacity());
    map.shrink_to_fit();
    println!("len: {}, capacity: {}", map.len(), map.capacity());
}

fn extend() {
    print_func_name!();
    let mut map: HashMap<_, _> = vec![("a", 1), ("b", 2)].into_iter().collect();
    println!("origin: {:?}", map);
    let more = [("c", 3), ("d", 4)];
    map.extend(more);
    println!("extend: {:?}", map);
    println!();
}

fn main() {
    create();
    insert_and_update();
    get_from_hash_map();
    remove();
    traverse();
    len_and_cap();
    extend();
}