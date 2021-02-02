use proconio::input;

fn main() {
    const INF: i32 = 20000000;
    // 入力を受け取る
    input! {
        N: usize,
        K: usize,
        a: [i32; N],
        b: [i32; N],
    }

    //　線形探索
    let mut min_value = INF;
    for i in 0..N {
        for j in 0..N {
            // 和がK未満の場合は除外する
            if a[i] + b[j] < K as i32 {
                continue;
            }

            // 最小値を更新する
            if a[i] + b[j] < min_value {
                min_value = a[i] + b[j];
            }
        }
    }

    println!("{:?}", min_value)
}
