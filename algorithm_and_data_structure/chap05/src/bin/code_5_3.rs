use proconio::input;
use std::collections::HashMap;

fn chmin(a: &mut i64, b: i64) {
    if *a > b {
        *a = b;
    }
}

fn main() {
    // 入力
    // input! {
    //     N: usize,
    //     h: [i64; N],
    // }
    let N: usize = 7;
    let h: [i64; 7] = [2, 9, 4, 5, 1, 6, 10];

    // 十分な大きさの数値で初期化する
    let mut dp = vec![100000; N];
    dp[0] = 0;

    for i in 1..N {
        let tmp = dp[i - 1] + (h[i] - h[i - 1]).abs();
        chmin(&mut dp[i], tmp);
        if i > 1 {
            let tmp = dp[i - 2] + (h[i] - h[i - 2]).abs();
            chmin(&mut dp[i], tmp);
        }
    }

    println!("{:?}", dp[N - 1]);
}
