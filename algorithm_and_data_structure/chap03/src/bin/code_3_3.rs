use proconio::input;

fn main() {
    // 入力を受け取る
    input! {
        N: usize,
        a: [i64; N],
    }
    const INF: i64 = 200000000;

    let mut min_value = INF;
    // 線形探索
    for i in 0..N {
        if a[i] < min_value {
            min_value = a[i]
        }
    }

    // 結果出力
    println!("{:?}", min_value)
}
