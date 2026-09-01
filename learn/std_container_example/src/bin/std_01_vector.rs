fn create() {
    println!("-----------------create-----------------");
    // 1. 使用 vec! 宏
    let v1 = vec![1, 2, 3];
    // 2. 空 Vec，指定类型
    let mut v2: Vec<i32> = Vec::new();
    // 3. 指定容量（避免频繁扩容）
    let mut v3: Vec<i32> = Vec::with_capacity(10);
    // 4. 重复元素初始化
    let v4 = vec![0; 5];

    println!("{:?}; {:?}; {:?}; {:?}", v1, v2, v3, v4);
    println!("len: {}; {}; {}; {}", v1.len(), v2.len(), v3.len(), v4.len());
    println!("cap: {}; {}; {}; {}", v1.capacity(), v2.capacity(), v3.capacity(), v4.capacity());
}

fn insert() {
    println!("-----------------insert-----------------");
    let mut v = vec![1, 2, 3];
    // 尾部追加: [1, 2, 3, 4]
    v.push(4);
    println!("push  : {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 尾部追加多个: [1, 2, 3, 4, 5, 6, 7]
    v.extend([5, 6, 7]);
    println!("extend: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 尾部追加多个: [1, 2, 3, 4, 5, 6, 7]
    v.extend_from_slice(&[5, 6, 7]);
    println!("extend_from_slice: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 复制一份追加到末尾: [1, 2, 3, 4, 1， 2， 3， 4]
    // 参数是: [start_index, end_index)
    // extend_from_within(start_index..end_index)
    // 变体1: extend_from_within(..)
    // 变体2: extend_from_within(start_index..)
    // 变体3: extend_from_within(..end_index)
    // 变体4: extend_from_within(start_index..end_index)
    v.extend_from_within(..);
    println!("extend_from_within: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 复制追加到末尾: [1, 2, 3, 4， 2， 3， 4]
    v.extend_from_within(1..);
    println!("extend_from_within: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 复制追加到末尾: [1, 2, 3, 4, 1， 2]
    v.extend_from_within(..2);
    println!("extend_from_within: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // 复制追加到末尾: [1, 2, 3, 4, 2]
    v.extend_from_within(1..2);
    println!("extend_from_within: {:?}", v);

    let mut v = vec![1, 2, 3, 4];
    // insert(index, value)
    v.insert(0, 0);
    println!("insert: {:?}", v);
}

fn extend() {
    println!("-----------------extend-----------------");
    let mut v = vec![1, 2, 3, 4];
    println!("before reserve: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());
    v.reserve(10);
    println!("before reserve: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());

    let mut v = vec![1, 2, 3, 4];
    println!("before reserve_exact: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());
    v.reserve_exact(10);
    println!("before reserve_exact: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());

    let mut v = vec![1, 2, 3, 4];
    println!("before resize: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());
    v.resize(10, 0);
    println!("before resize: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());
}

fn remove() {
    println!("-----------------remove-----------------");
    let mut v = vec![1, 2, 3, 4, 5];
    println!("原始数据   : {:?}", v);
    // 删除并返回尾部元素
    // Some(5), v = [1,2,3,4]
    let last = v.pop();
    println!("删除最后一个: {:?}", v);
    match last {
        None => {}
        Some(v) => { println!("最后一个元素: {}", v) }
    }
    // 删除指定位置元素
    // removed = 2, v = [1,3,4]
    let removed = v.remove(1);
    println!("删除指定元素: {:?}", v);
    println!("删除指定元素: {:?}", removed);
    // 删除指定范围
    // v.drain(start_index..end_index)
    // 删除区间: [start_index, end_index)
    // 删除索引 0，v = [1, 4, 5]
    let mut v = vec![1, 2, 3, 4, 5];
    v.drain(1..3);
    println!("删除指定范围: {:?}", v);
    // 清空
    // v = []
    v.clear();
    println!("清空: {:?}, len:{}, cap:{}", v, v.len(), v.capacity());
    // 按条件删除，或者叫按条件保留（retain 保留符合条件的）
    // 只保留偶数 → [2, 4, 6]
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0);
    println!("按条件删除: {:?}", v);
}

fn get_from_vec() {
    println!("-----------------get_from_vec-----------------");
    let v = vec![1, 2, 3, 4, 5];
    // 1. 索引访问（越界会 panic）
    let a = v[1];
    println!("v[1]:      {}", a);
    // 2. get 方法（安全，返回 Option）
    let b = v.get(1);
    let c = v.get(10);
    println!("v.get(1):  {:?}", b);
    println!("v.get(10): {:?}", c);
    // 3. 首尾元素
    let first = v.first();        // Some(&1)
    let last = v.last();          // Some(&5)
    println!("v.first:   {:?}", first);
    println!("v.last:    {:?}", last);
    // 4. 切片
    let slice = &v[1..3];         // [2, 3]
    println!("v[1..3]:   {:?}", slice);
}

fn len_and_cap() {
    println!("-----------------len_and_cap-----------------");
    let mut v = Vec::with_capacity(10);
    v.push(1);
    v.push(2);

    println!("before shrink_to_fit: {:?}", v);
    println!("len:      {}", v.len());        // 2
    println!("capacity: {}", v.capacity()); // 10
    println!("is_empty: {}", v.is_empty()); // false

    v.shrink_to_fit();  // 释放多余容量，capacity ≈ len
    println!("after shrink_to_fit:  {:?}", v);
    println!("len:      {}", v.len());        // 2
    println!("capacity: {}", v.capacity()); // 10
    println!("is_empty: {}", v.is_empty()); // false
}

fn traverse() {
    println!("-----------------traverse-----------------");
    let mut v = vec![1, 2, 3, 4, 5];

    // 不可变借用遍历
    println!("不可变借用遍历");
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // 可变借用遍历（可修改）
    println!("可变借用遍历（可修改）");
    for x in &mut v {
        *x *= 2;
    }
    println!("{:?}", v); // [2, 4, 6, 8, 10]

    // 遍历的同时获取索引
    println!("遍历的同时获取索引");
    for (i, x) in v.iter().enumerate() {
        println!("index: {}, value: {}", i, x);
    }

    // into_iter 消费 Vec
    println!("into_iter 消费 Vec, 已经被移动，不能再使用");
    for x in v {
        print!("{} ", x);
    }
    println!();
    // v 在这里已经被移动，不能再使用
}

fn find() {
    println!("-----------------find-----------------");
    let v = vec![1, 3, 5, 7, 9];

    // 是否包含某元素
    let has = v.contains(&3);     // true
    println!("v.contains: {}", has);

    // 查找位置
    let pos1 = v.iter().position(|&x| x == 5); // Some(2)
    let pos2 = v.iter().position(|&x| x == 6);
    println!("v.iter().position: {:?}", pos1);
    println!("v.iter().position: {:?}", pos2);

    // 二分查找（⚠️ 必须是有序 Vec）
    let mut v2 = vec![1, 2, 3, 4, 5];
    // 排序
    v2.sort();
    let idx = v2.binary_search(&3); // Ok(2)
    println!("v2.binary_search: {:?}", idx);

    // 所有元素满足条件
    let all_even = v.iter().all(|&x| x % 2 != 0); // true
    println!("v.iter().all: {}", all_even);

    // 任一元素满足条件
    let any_gt_4 = v.iter().any(|&x| x > 4);      // true
    println!("v.iter().any: {}", any_gt_4);
}

fn sort() {
    println!("-----------------sort-----------------");
    let mut v = vec![5, 2, 8, 1, 9];
    println!("原始: {:?}", v);

    // 升序
    v.sort();                     // [1, 2, 5, 8, 9]
    println!("升序: {:?}", v);

    // 降序
    v.sort_by(|a, b| b.cmp(a));  // [9, 8, 5, 2, 1]
    println!("降序: {:?}", v);

    // 反转
    v.reverse();                  // [1, 2, 5, 8, 9]
    println!("反转: {:?}", v);

    // 自定义类型排序
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 20 },
    ];

    println!("原始: {:?}", people);
    people.sort_by_key(|p| p.age); // 按 age 升序
    println!("排序: {:?}", people);
}

fn append_and_split() {
    println!("-----------------append_and_split-----------------");
    let mut a = vec![1, 2, 3];
    let mut b = vec![4, 5, 6];

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // 拼接
    a.append(&mut b);   // a = [1,2,3,4,5,6], b = []
    println!("a.append(&mut b): {:?}", a);

    // 拆分（按条件）
    let v = vec![1, 2, 3, 4, 5, 6];
    // evens = [2, 4, 6]
    let evens: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // split_at（按索引拆分切片）
    let v = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);
    // left = [1, 2], right = [3, 4, 5]
    let (left, right) = v.split_at(2);
    println!("left:  {:?}", left);
    println!("right: {:?}", right);
}

// 常用函数汇总速查表
// push:                    尾部追加
// pop:                     删除并返回尾部元素
// insert:                  指定位置插入
// remove:                  删除指定位置元素
// drain:                   删除指定范围
// clear:                   清空
// extend:                  追加多个元素
// retain:                  按条件保留
// get:                     安全访问
// first / last:            首尾元素
// len / capacity:          长度/容量
// is_empty:                是否为空
// contains:                是否包含
// sort / sort_by:          排序
// reverse:                 反转
// binary_search:           二分查找
// append:                  拼接另一个 Vec
// reserve / shrink_to_fit: 容量管理
// iter / into_iter:        遍历
// collect:                 迭代器 → Vec

fn main() {
    create();
    insert();
    extend();
    remove();
    get_from_vec();
    len_and_cap();
    traverse();
    find();
    sort();
    append_and_split();
}