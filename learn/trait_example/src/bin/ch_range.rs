fn main() {
    // ..    全范围   (-inf, +inf)
    // ..b   右开     (-inf, b)    索引场景下自动变为 [0, b)
    // ..=b  右闭     (-inf, b]    索引场景下自动变为 [0, b]
    // a..b  左闭右开  [a, b)
    // a..=b 左右都闭  [a, b]
    println!("Hello, world!");
}