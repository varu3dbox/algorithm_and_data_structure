use proconio::input;
use std::collections::HashMap;

fn chmin(a: &mut i64, b: i64) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

fn main() {
    // 入力
    // input! {
    //     N: usize,
    //     h: [i64; N],
    // }
    let N = 7;
    let h: [i64; 7] = [2, 9, 4, 5, 1, 6, 10];

    let mut dp = HashMap::new();
    dp.insert(0, 0);

    for i in 1..N {
        let mut tmp = dp[&(i - 1)];
        chmin(&mut tmp, dp[&(i - 1)] + (h[i] - h[i - 1]).abs());

        if i > 1 {
            let mut tmp = dp[&i];
            chmin(
                &mut tmp,
                dp.get(&(i - 2)).unwrap() + (h[i] - h[i - 2]).abs(),
            );
        }
    }

    println!("{:?}", dp[&(N - 1)])
}
