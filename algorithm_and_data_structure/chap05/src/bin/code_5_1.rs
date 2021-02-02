use proconio::input;
use std::collections::HashMap;

fn main() {
    // 入力
    // input! {
    //     N: usize,
    //     h: [i64; N],
    // }
    let N = 7;
    let h: [i64; 7] = [2, 9, 4, 5, 1, 6, 10];

    // メモ化用HashMap
    let mut dp: HashMap<usize, i64> = HashMap::new();
    dp.insert(0, 0);

    for i in 1..N {
        if i == 1 {
            dp.insert(i, (h[i] - h[i - 1]).abs());
        } else {
            let min_1 = dp.get(&(i - 1)).unwrap() + (h[i] - h[i - 1]).abs();
            let min_2 = dp.get(&(i - 2)).unwrap() + (h[i] - h[i - 2]).abs();
            if min_1 < min_2 {
                dp.insert(i, min_1);
            } else {
                dp.insert(i, min_2);
            }
        }
    }

    println!("{:?}", dp[&(N - 1)])
}
