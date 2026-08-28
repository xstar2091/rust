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

fn main() {
    create();
    insert();
    extend();
}