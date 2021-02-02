fn GCD(m: i32, n: i32) -> i32 {
    // ベースケース
    if n == 0 {
        return m;
    }

    // 再帰呼び出し
    return GCD(n, m % n);
}

fn main() {
    println!("{:?}", GCD(51, 15));
    println!("{:?}", GCD(15, 51));
}
